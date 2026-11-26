//! Authentication, session lifecycle, onboarding, scope switching, and sync actions for `RootView`.

use gpui_kit::{Context, Window};
#[cfg(not(target_arch = "wasm32"))]
use umuryango_core_domain::SyncStatus;
use umuryango_core_domain::models::FamilyRole;
use umuryango_core_domain::scope::LedgerScope;
use umuryango_i18n::Locale;
#[cfg(not(target_arch = "wasm32"))]
use umuryango_i18n::{Translator, terms};
use uuid::Uuid;

use crate::RootView;
#[cfg(not(target_arch = "wasm32"))]
use crate::api_bridge::ApiBridge;
use crate::state::{OnboardingStep, Screen};

#[cfg(not(target_arch = "wasm32"))]
use umuryango_api_client::{
    AuthResponse, CreateFamilyRequest, JoinFamilyRequest, LoginRequest, RegisterRequest,
};

impl RootView {
    pub fn toggle_locale(&mut self, cx: &mut Context<Self>) {
        let new_locale = match self.state.locale {
            Locale::Rw => Locale::En,
            Locale::En => Locale::Rw,
        };
        self.state.switch_locale(new_locale);

        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some(ref db) = self.db
                && let Ok(locked) = db.lock()
            {
                self.state.save_session_to_db(&locked);
                self.state.save_navigation_to_db(&locked);
            }
            let client = self.api.client.clone();
            let is_auth = self.state.is_authenticated;
            let fut = self.api.run(async move {
                client.set_locale(new_locale).await;
                if is_auth {
                    let _ = client.update_locale(new_locale).await;
                }
            });
            cx.spawn(async move |_, _| {
                fut.await;
            })
            .detach();
        }

        cx.notify();
    }

    pub fn switch_to_family_scope(&mut self, cx: &mut Context<Self>) {
        self.state.families_menu_open = false;
        self.state.show_command_palette = false;
        if self.state.active_scope.is_family() {
            cx.notify();
            return;
        }
        let target = self
            .state
            .active_family_id
            .and_then(|fid| self.state.families.iter().find(|f| f.id == fid))
            .or_else(|| self.state.families.first())
            .cloned();

        if let Some(fam) = target {
            self.switch_to_family_by_id(fam.id, cx);
        } else {
            self.select_screen(Screen::Onboarding, cx);
        }
    }

    pub fn switch_to_family_by_id(&mut self, family_id: Uuid, cx: &mut Context<Self>) {
        self.state.families_menu_open = false;
        self.state.show_command_palette = false;
        if self.state.active_scope.family_id() == Some(family_id) {
            cx.notify();
            return;
        }
        let Some(fam) = self
            .state
            .families
            .iter()
            .find(|f| f.id == family_id)
            .cloned()
        else {
            cx.notify();
            return;
        };

        let db_lock = self.db.as_ref().and_then(|d| d.lock().ok());
        self.state.switch_scope(
            LedgerScope::Family { family_id: fam.id },
            Some(fam.name),
            fam.invite_code,
            fam.role,
            db_lock.as_deref(),
        );
        drop(db_lock);
        self.sync_default_wallets();
        cx.notify();
        self.refresh_active_scope(cx);
    }

    pub fn switch_to_personal_scope(&mut self, cx: &mut Context<Self>) {
        self.state.families_menu_open = false;
        self.state.show_command_palette = false;
        if !self.state.active_scope.is_family() {
            cx.notify();
            return;
        }
        let db_lock = self.db.as_ref().and_then(|d| d.lock().ok());
        let scope = LedgerScope::Personal {
            user_id: self.state.user_id,
        };
        self.state
            .switch_scope(scope, None, None, FamilyRole::Member, db_lock.as_deref());
        drop(db_lock);
        self.sync_default_wallets();
        cx.notify();
        self.refresh_active_scope(cx);
    }

    pub fn sync_default_wallets(&mut self) {
        let wallets = self.state.active_wallets();
        self.quick_entry_form.source_wallet_id = wallets.first().map(|w| w.id);
        self.quick_entry_form.destination_wallet_id = wallets.get(1).map(|w| w.id);
    }

    pub fn handle_logout(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let locale = self.state.locale;
        self.state.reset_session();

        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some(ref db) = self.db
                && let Ok(locked) = db.lock()
            {
                crate::state::AppState::clear_session_in_db(&locked);
                self.state.save_navigation_to_db(&locked);
            }
            let client = self.api.client.clone();
            let fut = self.api.run(async move {
                client.set_token(None).await;
            });
            cx.spawn(async move |_, _| {
                fut.await;
            })
            .detach();
        }

        self.auth_inputs.clear_passwords(window, cx);
        self.onboarding_inputs.clear_all(window, cx);
        self.quick_entry_inputs.reset(window, cx);
        self.entity_modal_inputs.reset(window, cx);
        self.state
            .set_toast(umuryango_i18n::t(locale, "auth.logged_out"), false);
        cx.notify();
    }

    pub(crate) fn handle_session_expired(&mut self, cx: &mut Context<Self>) {
        let locale = self.state.locale;
        self.state.reset_session();

        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some(ref db) = self.db
                && let Ok(locked) = db.lock()
            {
                crate::state::AppState::clear_session_in_db(&locked);
                self.state.save_navigation_to_db(&locked);
            }
            let client = self.api.client.clone();
            let fut = self.api.run(async move {
                client.set_token(None).await;
            });
            cx.spawn(async move |_, _| {
                fut.await;
            })
            .detach();
        }

        self.state
            .set_toast(umuryango_i18n::t(locale, "auth.session_expired"), true);
        cx.notify();
    }

    /// Refreshes all data for the currently active scope directly from PostgreSQL via `api-server`.
    pub fn refresh_active_scope(&mut self, cx: &mut Context<Self>) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            if !self.state.is_authenticated {
                return;
            }
            let client = self.api.client.clone();
            let scope = self.state.active_scope;
            let db = self.db.clone();
            let locale = self.state.locale;

            let fut = self
                .api
                .run(async move { ApiBridge::fetch_scope_snapshot(client, scope, db).await });

            cx.spawn(async move |this, cx| {
                let res = fut.await;
                let _ = this.update(cx, |this, cx| {
                    match res {
                        Ok(snap) => {
                            this.state.apply_scope_snapshot(snap);
                            this.sync_default_wallets();
                            if let Some(ref db_arc) = this.db
                                && let Ok(locked) = db_arc.lock()
                            {
                                this.state.save_session_to_db(&locked);
                            }
                        }
                        Err(err) => {
                            if err.is_unauthorized() {
                                this.handle_session_expired(cx);
                                return;
                            }
                            if err.is_network_error() {
                                this.state.sync_status = SyncStatus::Offline;
                            }
                            this.state.set_toast(err.user_message(locale), true);
                        }
                    }
                    cx.notify();
                });
            })
            .detach();
        }
        #[cfg(target_arch = "wasm32")]
        {
            let _ = cx;
        }
    }

    /// Restores a previously saved JWT token on startup and verifies it against PostgreSQL.
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn verify_restored_session(&mut self, token: String, cx: &mut Context<Self>) {
        let client = self.api.client.clone();
        let locale = self.state.locale;
        let scope = self.state.active_scope;
        let db = self.db.clone();

        let fut = self.api.run(async move {
            client.set_token(Some(token)).await;
            client.set_locale(locale).await;
            ApiBridge::fetch_scope_snapshot(client, scope, db).await
        });

        cx.spawn(async move |this, cx| {
            let res = fut.await;
            let _ = this.update(cx, |this, cx| {
                match res {
                    Ok(snap) => {
                        this.state.apply_scope_snapshot(snap);
                        this.sync_default_wallets();
                    }
                    Err(err) => {
                        if err.is_unauthorized() {
                            this.handle_session_expired(cx);
                            return;
                        }
                        if err.is_network_error() {
                            this.state.sync_status = SyncStatus::Offline;
                        }
                    }
                }
                cx.notify();
            });
        })
        .detach();
    }

    pub fn handle_login_submit(&mut self, cx: &mut Context<Self>) {
        if self.state.is_busy {
            return;
        }
        let locale = self.state.locale;
        let phone = self.state.auth_phone.trim().to_string();
        let password = self.state.auth_password.clone();

        if phone.is_empty() {
            self.state
                .set_toast(umuryango_i18n::t(locale, "auth.phone_required"), true);
            cx.notify();
            return;
        }
        if password.is_empty() {
            self.state
                .set_toast(umuryango_i18n::t(locale, "auth.password_required"), true);
            cx.notify();
            return;
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            self.state.is_busy = true;
            cx.notify();

            let client = self.api.client.clone();
            let req = LoginRequest {
                phone: Some(phone.clone()),
                login: Some(phone.clone()),
                email: None,
                password,
            };
            let fut = self.api.run(async move {
                client.set_locale(locale).await;
                client.login(&req).await
            });

            cx.spawn(async move |this, cx| {
                let res = fut.await;
                let _ = this.update(cx, |this, cx| {
                    this.state.is_busy = false;
                    match res {
                        Ok(auth) => {
                            this.apply_auth_response(auth, phone, false, cx);
                        }
                        Err(err) => {
                            this.state.set_toast(err.user_message(locale), true);
                            cx.notify();
                        }
                    }
                });
            })
            .detach();
        }
        #[cfg(target_arch = "wasm32")]
        {
            self.state
                .set_toast(umuryango_i18n::t(locale, "errors.network_offline"), true);
            cx.notify();
        }
    }

    pub fn handle_register_submit(&mut self, cx: &mut Context<Self>) {
        if self.state.is_busy {
            return;
        }
        let locale = self.state.locale;
        let full_name = self.state.auth_full_name.trim().to_string();
        let phone = self.state.auth_phone.trim().to_string();
        let password = self.state.auth_password.clone();
        let confirm = self.state.auth_confirm_password.clone();
        #[cfg(not(target_arch = "wasm32"))]
        let gender = self.state.auth_gender.clone();

        if full_name.is_empty() {
            self.state
                .set_toast(umuryango_i18n::t(locale, "auth.full_name_required"), true);
            cx.notify();
            return;
        }
        if phone.is_empty() {
            self.state
                .set_toast(umuryango_i18n::t(locale, "auth.phone_required"), true);
            cx.notify();
            return;
        }
        if password.len() < 6 {
            self.state
                .set_toast(umuryango_i18n::t(locale, "auth.password_too_short"), true);
            cx.notify();
            return;
        }
        if password != confirm {
            self.state
                .set_toast(umuryango_i18n::t(locale, "auth.passwords_mismatch"), true);
            cx.notify();
            return;
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            self.state.is_busy = true;
            cx.notify();

            let client = self.api.client.clone();
            let req = RegisterRequest {
                full_name,
                phone: Some(phone.clone()),
                phone_number: Some(phone.clone()),
                gender: Some(gender),
                password,
                confirm_password: Some(confirm),
                email: None,
                preferred_locale: Some(locale),
            };
            let fut = self.api.run(async move {
                client.set_locale(locale).await;
                client.register(&req).await
            });

            cx.spawn(async move |this, cx| {
                let res = fut.await;
                let _ = this.update(cx, |this, cx| {
                    this.state.is_busy = false;
                    match res {
                        Ok(auth) => {
                            this.apply_auth_response(auth, phone, true, cx);
                        }
                        Err(err) => {
                            this.state.set_toast(err.user_message(locale), true);
                            cx.notify();
                        }
                    }
                });
            })
            .detach();
        }
        #[cfg(target_arch = "wasm32")]
        {
            self.state
                .set_toast(umuryango_i18n::t(locale, "errors.network_offline"), true);
            cx.notify();
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn apply_auth_response(
        &mut self,
        auth: AuthResponse,
        fallback_phone: String,
        is_new_account: bool,
        cx: &mut Context<Self>,
    ) {
        let user_id = auth
            .user
            .as_ref()
            .map(|u| u.id)
            .or(auth.user_id)
            .unwrap_or_else(Uuid::nil);
        let user_name = auth
            .user
            .as_ref()
            .map(|u| u.full_name.clone())
            .or(auth.full_name)
            .unwrap_or_default();
        let user_phone = auth
            .user
            .as_ref()
            .and_then(|u| u.phone_number.clone())
            .unwrap_or(fallback_phone);

        if let Some(loc) = auth
            .user
            .as_ref()
            .map(|u| u.preferred_locale)
            .or(auth.locale)
        {
            self.state.locale = loc;
        }

        self.state.user_id = user_id;
        self.state.user_name = user_name;
        self.state.user_phone = user_phone;
        self.state.auth_token = Some(auth.token);
        self.state.is_authenticated = true;
        self.state.auth_mode_register = false;
        self.state.onboarding_step = OnboardingStep::Choose;
        self.state.auth_password.clear();
        self.state.auth_confirm_password.clear();
        self.state.families = auth.families;
        self.state.pending_invites = auth.pending_invites;

        if !is_new_account && let Some(first_fam) = self.state.families.first().cloned() {
            self.state.active_scope = LedgerScope::Family {
                family_id: first_fam.id,
            };
            self.state.active_family_id = Some(first_fam.id);
            self.state.active_family_name = Some(first_fam.name);
            self.state.active_family_invite_code = first_fam.invite_code;
            self.state.active_role = first_fam.role;
            self.state.current_screen = Screen::Dashboard;
        } else {
            self.state.active_scope = LedgerScope::Personal { user_id };
            self.state.active_family_id = None;
            self.state.active_family_name = None;
            self.state.active_family_invite_code = None;
            self.state.active_role = FamilyRole::Member;
            self.state.current_screen = if is_new_account || !self.state.pending_invites.is_empty()
            {
                Screen::Onboarding
            } else {
                Screen::Dashboard
            };
        }

        if let Some(ref db) = self.db
            && let Ok(locked) = db.lock()
        {
            self.state.save_session_to_db(&locked);
            self.state.save_navigation_to_db(&locked);
        }

        let toast_key = if is_new_account {
            "auth.account_created"
        } else {
            "auth.login_success"
        };
        self.state
            .set_toast(umuryango_i18n::t(self.state.locale, toast_key), false);
        cx.notify();
        self.refresh_active_scope(cx);
    }

    pub fn add_onboarding_phone(&mut self, cx: &mut Context<Self>) {
        let candidate = self.state.onboarding_phone_draft.trim().to_string();
        if !candidate.is_empty() && !self.state.onboarding_member_phones.contains(&candidate) {
            self.state.onboarding_member_phones.push(candidate);
        }
        self.state.onboarding_phone_draft.clear();
        cx.notify();
    }

    pub fn add_onboarding_phone_with_window(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.add_onboarding_phone(cx);
        self.onboarding_inputs.clear_member_phone(window, cx);
    }

    pub fn remove_onboarding_phone(&mut self, index: usize, cx: &mut Context<Self>) {
        if index < self.state.onboarding_member_phones.len() {
            self.state.onboarding_member_phones.remove(index);
            cx.notify();
        }
    }

    pub fn handle_create_family_submit(&mut self, cx: &mut Context<Self>) {
        if self.state.is_busy {
            return;
        }
        let locale = self.state.locale;
        let name = self.state.onboarding_family_name.trim().to_string();
        if name.is_empty() {
            self.state.set_toast(
                umuryango_i18n::t(locale, "onboarding.family_name_required"),
                true,
            );
            cx.notify();
            return;
        }

        let mut phones = self.state.onboarding_member_phones.clone();
        let draft = self.state.onboarding_phone_draft.trim().to_string();
        if !draft.is_empty() && !phones.contains(&draft) {
            phones.push(draft);
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            self.state.is_busy = true;
            cx.notify();

            let client = self.api.client.clone();
            let req = CreateFamilyRequest {
                name,
                description: None,
                currency: Some("RWF".to_string()),
                member_phones: phones,
            };
            let fut = self
                .api
                .run(async move { client.create_family_detailed(&req).await });

            cx.spawn(async move |this, cx| {
                let res = fut.await;
                let _ = this.update(cx, |this, cx| {
                    this.state.is_busy = false;
                    match res {
                        Ok(created) => {
                            let fam = created.family;
                            let code = created.invite_code;
                            let db_lock = this.db.as_ref().and_then(|d| d.lock().ok());
                            this.state.switch_scope(
                                LedgerScope::Family { family_id: fam.id },
                                Some(fam.name),
                                Some(code.clone()),
                                FamilyRole::Head,
                                db_lock.as_deref(),
                            );
                            drop(db_lock);
                            this.state.onboarding_step = OnboardingStep::Choose;
                            this.state.onboarding_family_name.clear();
                            this.state.onboarding_phone_draft.clear();
                            this.state.onboarding_member_phones.clear();
                            this.state.current_screen = Screen::Dashboard;
                            this.persist_navigation();
                            this.state.set_toast(
                                umuryango_i18n::t_args(
                                    locale,
                                    "onboarding.family_created_code",
                                    &[("code", &code)],
                                ),
                                false,
                            );
                            cx.notify();
                            this.refresh_active_scope(cx);
                        }
                        Err(err) => {
                            if err.is_unauthorized() {
                                this.handle_session_expired(cx);
                                return;
                            }
                            this.state.set_toast(err.user_message(locale), true);
                            cx.notify();
                        }
                    }
                });
            })
            .detach();
        }
        #[cfg(target_arch = "wasm32")]
        {
            let _ = phones;
            self.state
                .set_toast(umuryango_i18n::t(locale, "errors.network_offline"), true);
            cx.notify();
        }
    }

    pub fn handle_join_family_submit(&mut self, cx: &mut Context<Self>) {
        if self.state.is_busy {
            return;
        }
        let locale = self.state.locale;
        let code = self.state.onboarding_join_code.trim().to_ascii_uppercase();
        if code.len() != 4 {
            self.state.set_toast(
                umuryango_i18n::t(locale, "errors.invalid_invite_code"),
                true,
            );
            cx.notify();
            return;
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            self.join_family_request_async(
                JoinFamilyRequest {
                    invite_code: Some(code),
                    invite_id: None,
                    display_name: None,
                    role: None,
                },
                None,
                cx,
            );
        }
        #[cfg(target_arch = "wasm32")]
        {
            self.state
                .set_toast(umuryango_i18n::t(locale, "errors.network_offline"), true);
            cx.notify();
        }
    }

    pub fn handle_accept_invite(&mut self, invite_id: Uuid, cx: &mut Context<Self>) {
        if self.state.is_busy {
            return;
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.join_family_request_async(
                JoinFamilyRequest {
                    invite_code: None,
                    invite_id: Some(invite_id),
                    display_name: None,
                    role: None,
                },
                Some(invite_id),
                cx,
            );
        }
        #[cfg(target_arch = "wasm32")]
        {
            let _ = invite_id;
            let locale = self.state.locale;
            self.state
                .set_toast(umuryango_i18n::t(locale, "errors.network_offline"), true);
            cx.notify();
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn join_family_request_async(
        &mut self,
        req: JoinFamilyRequest,
        accepted_invite_id: Option<Uuid>,
        cx: &mut Context<Self>,
    ) {
        let locale = self.state.locale;
        self.state.is_busy = true;
        cx.notify();

        let client = self.api.client.clone();
        let fut = self
            .api
            .run(async move { client.join_family_detailed(&req).await });

        cx.spawn(async move |this, cx| {
            let res = fut.await;
            let _ = this.update(cx, |this, cx| {
                this.state.is_busy = false;
                match res {
                    Ok(joined) => {
                        if let Some(inv_id) = accepted_invite_id {
                            this.state.pending_invites.retain(|i| i.id != inv_id);
                        }
                        let db_lock = this.db.as_ref().and_then(|d| d.lock().ok());
                        this.state.switch_scope(
                            LedgerScope::Family {
                                family_id: joined.family_id,
                            },
                            Some(joined.family_name),
                            joined.family.invite_code,
                            joined.role,
                            db_lock.as_deref(),
                        );
                        drop(db_lock);
                        this.state.onboarding_step = OnboardingStep::Choose;
                        this.state.onboarding_join_code.clear();
                        this.state.current_screen = Screen::Dashboard;
                        this.persist_navigation();
                        this.state.set_toast(
                            Translator::t(locale, terms::CONTINUE_TO_DASHBOARD, None),
                            false,
                        );
                        cx.notify();
                        this.refresh_active_scope(cx);
                    }
                    Err(err) => {
                        if err.is_unauthorized() {
                            this.handle_session_expired(cx);
                            return;
                        }
                        this.state.set_toast(err.user_message(locale), true);
                        cx.notify();
                    }
                }
            });
        })
        .detach();
    }

    pub fn handle_skip_onboarding(&mut self, cx: &mut Context<Self>) {
        let db_lock = self.db.as_ref().and_then(|d| d.lock().ok());
        self.state.switch_scope(
            LedgerScope::Personal {
                user_id: self.state.user_id,
            },
            None,
            None,
            FamilyRole::Member,
            db_lock.as_deref(),
        );
        drop(db_lock);
        self.state.onboarding_step = OnboardingStep::Choose;
        self.state.current_screen = Screen::Dashboard;
        self.persist_navigation();
        cx.notify();
        self.refresh_active_scope(cx);
    }
}
