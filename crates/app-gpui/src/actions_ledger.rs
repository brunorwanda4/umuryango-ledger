//! Ledger mutation actions for `RootView`: Quick Entry transactions,
//! wallets, projects, loans, repayments, future plans, allocations,
//! member invites/roles, and zero-login report magic links.

use std::str::FromStr;

use gpui_kit::{Context, Window};
use rust_decimal::Decimal;
use umuryango_core_domain::models::FamilyRole;
#[cfg(not(target_arch = "wasm32"))]
use umuryango_core_domain::models::TransactionType;
use uuid::Uuid;

use crate::RootView;
use crate::components::quick_entry::{QuickEntryDialog, QuickEntryMode};
use crate::screens::dashboard::metrics::DashboardPeriod;
use crate::state::{ActiveModal, EntityFormState};

#[cfg(not(target_arch = "wasm32"))]
use umuryango_api_client::{
    CreateLoanRequest, CreatePlanRequest, CreateProjectRequest, CreateTransactionRequest,
    CreateWalletRequest,
};

impl RootView {
    pub fn set_dashboard_period(&mut self, period: DashboardPeriod, cx: &mut Context<Self>) {
        if self.state.dashboard_period != Some(period) {
            self.state.dashboard_period = Some(period);
            cx.notify();
        }
    }

    pub fn open_quick_handover(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.open_quick_entry(window, cx);
        if self.state.active_scope.is_family() {
            self.set_quick_entry_mode(QuickEntryMode::Handover, cx);
        }
    }

    pub fn open_quick_entry(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.quick_entry_form.source_wallet_id.is_none() {
            self.quick_entry_form.source_wallet_id =
                self.state.active_wallets().first().map(|w| w.id);
        }
        if self.quick_entry_form.destination_wallet_id.is_none() {
            self.quick_entry_form.destination_wallet_id =
                self.state.active_wallets().get(1).map(|w| w.id);
        }
        self.quick_entry_form.amount_input.clear();
        self.quick_entry_form.description.clear();
        self.quick_entry_form.paid_to.clear();
        self.quick_entry_inputs.reset(window, cx);
        self.state.show_quick_entry = true;
        cx.notify();
    }

    pub fn close_quick_entry(&mut self, cx: &mut Context<Self>) {
        self.state.show_quick_entry = false;
        cx.notify();
    }

    pub fn set_quick_entry_mode(&mut self, mode: QuickEntryMode, cx: &mut Context<Self>) {
        self.quick_entry_form.mode = mode;
        if mode == QuickEntryMode::Handover && self.quick_entry_form.destination_wallet_id.is_none()
        {
            self.quick_entry_form.destination_wallet_id = self
                .state
                .active_wallets()
                .iter()
                .find(|w| Some(w.id) != self.quick_entry_form.source_wallet_id)
                .map(|w| w.id);
        }
        cx.notify();
    }

    pub fn set_quick_entry_amount_preset(
        &mut self,
        amount: String,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.quick_entry_form.amount_input = amount.clone();
        self.quick_entry_inputs.set_amount(&amount, window, cx);
        cx.notify();
    }

    pub fn submit_quick_entry(&mut self, cx: &mut Context<Self>) {
        if self.state.is_busy {
            return;
        }
        let locale = self.state.locale;
        let actor_id = self.state.user_id;
        let tx = match QuickEntryDialog::build_transaction(
            &self.quick_entry_form,
            &self.state.active_scope,
            actor_id,
            locale,
        ) {
            Ok(tx) => tx,
            Err(err) => {
                self.state.set_toast(err, true);
                cx.notify();
                return;
            }
        };

        #[cfg(not(target_arch = "wasm32"))]
        {
            self.state.is_busy = true;
            cx.notify();

            let client = self.api.client.clone();
            let scope = self.state.active_scope;
            let actor_role = self.state.active_role;
            let req = CreateTransactionRequest {
                scope: scope.as_str().to_string(),
                family_id: scope.family_id(),
                source_wallet_id: tx.source_wallet_id,
                destination_wallet_id: if tx.tx_type == TransactionType::Handover {
                    tx.destination_wallet_id
                } else {
                    None
                },
                tx_type: tx.tx_type,
                amount: tx.amount,
                category: tx.category.clone(),
                description: tx.description.clone(),
                paid_to: tx.paid_to.clone(),
                receipt_url: None,
                tx_date: Some(tx.tx_date),
                idempotency_key: Some(tx.id),
            };

            let fut = self
                .api
                .run(async move { client.create_transaction(&req).await });

            cx.spawn(async move |this, cx| {
                let res = fut.await;
                let _ = this.update(cx, |this, cx| {
                    this.state.is_busy = false;
                    match res {
                        Ok(result) => {
                            for w in &mut this.state.wallets {
                                if w.id == result.transaction.source_wallet_id {
                                    w.balance = result.source_wallet_balance;
                                }
                                if Some(w.id) == result.transaction.destination_wallet_id
                                    && let Some(dest_bal) = result.destination_wallet_balance
                                {
                                    w.balance = dest_bal;
                                }
                            }
                            this.state.transactions.insert(0, result.transaction);
                            this.state.show_quick_entry = false;
                            this.state.set_toast(
                                umuryango_i18n::t(locale, "transactions.transaction_recorded"),
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
                            if err.is_network_error() {
                                let db_lock = this.db.as_ref().and_then(|d| d.lock().ok());
                                match this.state.log_transaction_offline(
                                    tx,
                                    actor_role,
                                    db_lock.as_deref(),
                                ) {
                                    Ok(()) => {
                                        this.state.show_quick_entry = false;
                                        this.state.set_toast(
                                            umuryango_i18n::t(locale, "transactions.saved_offline"),
                                            false,
                                        );
                                    }
                                    Err(dom_err) => {
                                        this.state.set_toast(dom_err.to_string(), true);
                                    }
                                }
                            } else {
                                this.state.set_toast(err.user_message(locale), true);
                            }
                            cx.notify();
                        }
                    }
                });
            })
            .detach();
        }
        #[cfg(target_arch = "wasm32")]
        {
            let _ = tx;
            self.state
                .set_toast(umuryango_i18n::t(locale, "errors.network_offline"), true);
            cx.notify();
        }
    }

    pub fn open_modal(&mut self, modal: ActiveModal, window: &mut Window, cx: &mut Context<Self>) {
        if matches!(modal, ActiveModal::AddProject) && !self.state.active_scope.is_family() {
            self.state.set_toast(
                umuryango_i18n::t(self.state.locale, "projects.family_scope_required"),
                true,
            );
            cx.notify();
            return;
        }
        self.state.entity_form = EntityFormState::default();
        self.entity_modal_inputs.reset(window, cx);
        self.state.active_modal = Some(modal);
        cx.notify();
    }

    pub fn close_modal(&mut self, cx: &mut Context<Self>) {
        self.state.active_modal = None;
        cx.notify();
    }

    pub fn submit_entity_modal(&mut self, cx: &mut Context<Self>) {
        if self.state.is_busy {
            return;
        }
        let Some(modal) = self.state.active_modal.clone() else {
            return;
        };
        let locale = self.state.locale;
        let form = self.state.entity_form.clone();
        let scope = self.state.active_scope;
        let primary = form.primary_text.trim().to_string();
        let secondary = opt_non_empty(&form.secondary_text);

        #[cfg(not(target_arch = "wasm32"))]
        match modal {
            ActiveModal::AddWallet => {
                if primary.is_empty() {
                    self.state.set_toast(
                        umuryango_i18n::t(locale, "wallets.wallet_name_required"),
                        true,
                    );
                    cx.notify();
                    return;
                }
                let req = CreateWalletRequest {
                    name: primary,
                    channel: form.wallet_channel,
                    currency: "RWF".to_string(),
                    account_number: secondary,
                    custodian_id: Some(self.state.user_id),
                    scope: scope.as_str().to_string(),
                    family_id: scope.family_id(),
                };
                let client = self.api.client.clone();
                self.run_modal_mutation(
                    "wallets.wallet_created",
                    self.api
                        .run(async move { client.create_wallet(&req).await.map(|_| ()) }),
                    cx,
                );
            }
            ActiveModal::AddProject => {
                let Some(family_id) = scope.family_id() else {
                    self.state.set_toast(
                        umuryango_i18n::t(locale, "projects.family_scope_required"),
                        true,
                    );
                    cx.notify();
                    return;
                };
                if primary.is_empty() {
                    self.state
                        .set_toast(umuryango_i18n::t(locale, "projects.name_required"), true);
                    cx.notify();
                    return;
                }
                let Ok(target_amount) = parse_positive_amount(&form.amount_text) else {
                    self.state
                        .set_toast(umuryango_i18n::t(locale, "errors.invalid_amount"), true);
                    cx.notify();
                    return;
                };
                let req = CreateProjectRequest {
                    family_id,
                    name: primary,
                    description: secondary,
                    target_amount,
                    start_date: None,
                    target_end_date: None,
                };
                let client = self.api.client.clone();
                self.run_modal_mutation(
                    "projects.project_created",
                    self.api
                        .run(async move { client.create_project(&req).await.map(|_| ()) }),
                    cx,
                );
            }
            ActiveModal::AddLoan => {
                if primary.is_empty() {
                    self.state.set_toast(
                        umuryango_i18n::t(locale, "loans.counterparty_required"),
                        true,
                    );
                    cx.notify();
                    return;
                }
                let Ok(principal_amount) = parse_positive_amount(&form.amount_text) else {
                    self.state
                        .set_toast(umuryango_i18n::t(locale, "errors.invalid_amount"), true);
                    cx.notify();
                    return;
                };
                let req = CreateLoanRequest {
                    scope: scope.as_str().to_string(),
                    family_id: scope.family_id(),
                    counterparty_name: primary,
                    principal_amount,
                    interest_rate: None,
                    due_date: None,
                    direction: form.loan_direction,
                    notes: secondary,
                };
                let client = self.api.client.clone();
                self.run_modal_mutation(
                    "loans.loan_created",
                    self.api
                        .run(async move { client.create_loan(&req).await.map(|_| ()) }),
                    cx,
                );
            }
            ActiveModal::RepayLoan { loan_id, .. } => {
                let Ok(amount) = parse_positive_amount(&form.amount_text) else {
                    self.state
                        .set_toast(umuryango_i18n::t(locale, "errors.invalid_amount"), true);
                    cx.notify();
                    return;
                };
                let client = self.api.client.clone();
                self.run_modal_mutation(
                    "loans.repayment_recorded",
                    self.api
                        .run(async move { client.repay_loan(loan_id, amount).await.map(|_| ()) }),
                    cx,
                );
            }
            ActiveModal::AddPlan => {
                if primary.is_empty() {
                    self.state.set_toast(
                        umuryango_i18n::t(locale, "future_plans.title_required"),
                        true,
                    );
                    cx.notify();
                    return;
                }
                let Ok(estimated_cost) = parse_positive_amount(&form.amount_text) else {
                    self.state
                        .set_toast(umuryango_i18n::t(locale, "errors.invalid_amount"), true);
                    cx.notify();
                    return;
                };
                let req = CreatePlanRequest {
                    scope: scope.as_str().to_string(),
                    family_id: scope.family_id(),
                    title: primary,
                    description: secondary,
                    estimated_cost,
                    target_date: None,
                };
                let client = self.api.client.clone();
                self.run_modal_mutation(
                    "future_plans.plan_created",
                    self.api
                        .run(async move { client.create_plan(&req).await.map(|_| ()) }),
                    cx,
                );
            }
            ActiveModal::AllocatePlan { plan_id, .. } => {
                let Ok(amount) = parse_positive_amount(&form.amount_text) else {
                    self.state
                        .set_toast(umuryango_i18n::t(locale, "errors.invalid_amount"), true);
                    cx.notify();
                    return;
                };
                let client = self.api.client.clone();
                self.run_modal_mutation(
                    "future_plans.funds_allocated",
                    self.api.run(
                        async move { client.allocate_plan(plan_id, amount).await.map(|_| ()) },
                    ),
                    cx,
                );
            }
            ActiveModal::InviteMember => {
                let Some(family_id) = scope.family_id() else {
                    self.state.set_toast(
                        umuryango_i18n::t(locale, "members.no_family_selected"),
                        true,
                    );
                    cx.notify();
                    return;
                };
                if primary.is_empty() {
                    self.state
                        .set_toast(umuryango_i18n::t(locale, "members.phone_required"), true);
                    cx.notify();
                    return;
                }
                let role = form.invite_role;
                let client = self.api.client.clone();
                self.run_modal_mutation(
                    "members.invite_sent",
                    self.api.run(async move {
                        client.invite_family_member(family_id, &primary, role).await
                    }),
                    cx,
                );
            }
        }

        #[cfg(target_arch = "wasm32")]
        {
            let _ = (modal, primary, secondary, scope);
            self.state
                .set_toast(umuryango_i18n::t(locale, "errors.network_offline"), true);
            cx.notify();
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn run_modal_mutation<F>(&mut self, success_key: &'static str, fut: F, cx: &mut Context<Self>)
    where
        F: std::future::Future<Output = Result<(), umuryango_api_client::ApiClientError>>
            + Send
            + 'static,
    {
        let locale = self.state.locale;
        self.state.is_busy = true;
        cx.notify();

        cx.spawn(async move |this, cx| {
            let res = fut.await;
            let _ = this.update(cx, |this, cx| {
                this.state.is_busy = false;
                match res {
                    Ok(()) => {
                        this.state.active_modal = None;
                        this.state
                            .set_toast(umuryango_i18n::t(locale, success_key), false);
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

    pub fn handle_update_member_role(
        &mut self,
        member_id: Uuid,
        next_role: FamilyRole,
        cx: &mut Context<Self>,
    ) {
        let locale = self.state.locale;
        let Some(family_id) = self.state.active_scope.family_id() else {
            return;
        };

        #[cfg(not(target_arch = "wasm32"))]
        {
            let client = self.api.client.clone();
            let fut = self.api.run(async move {
                client
                    .update_member_role(family_id, member_id, next_role)
                    .await
            });
            cx.spawn(async move |this, cx| {
                let res = fut.await;
                let _ = this.update(cx, |this, cx| match res {
                    Ok(()) => {
                        this.state
                            .set_toast(umuryango_i18n::t(locale, "members.role_updated"), false);
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
                });
            })
            .detach();
        }
        #[cfg(target_arch = "wasm32")]
        {
            let _ = (family_id, member_id, next_role);
            self.state
                .set_toast(umuryango_i18n::t(locale, "errors.network_offline"), true);
            cx.notify();
        }
    }

    pub fn handle_create_magic_link(&mut self, cx: &mut Context<Self>) {
        let locale = self.state.locale;
        let Some(family_id) = self.state.active_scope.family_id() else {
            self.state.set_toast(
                umuryango_i18n::t(locale, "report.family_required_for_link"),
                true,
            );
            cx.notify();
            return;
        };

        #[cfg(not(target_arch = "wasm32"))]
        {
            let client = self.api.client.clone();
            let fut = self
                .api
                .run(async move { client.create_share_link(family_id, locale, 7).await });

            cx.spawn(async move |this, cx| {
                let res = fut.await;
                let _ = this.update(cx, |this, cx| match res {
                    Ok(link) => {
                        let feedback = umuryango_i18n::t(locale, "report.link_copied");
                        this.copy_to_clipboard(link.share_url, feedback, cx);
                    }
                    Err(err) => {
                        if err.is_unauthorized() {
                            this.handle_session_expired(cx);
                            return;
                        }
                        this.state.set_toast(err.user_message(locale), true);
                        cx.notify();
                    }
                });
            })
            .detach();
        }
        #[cfg(target_arch = "wasm32")]
        {
            let _ = family_id;
            self.state
                .set_toast(umuryango_i18n::t(locale, "errors.network_offline"), true);
            cx.notify();
        }
    }
}

fn opt_non_empty(s: &str) -> Option<String> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn parse_positive_amount(raw: &str) -> Result<Decimal, ()> {
    let cleaned = raw.trim().replace([',', ' '], "");
    let dec = Decimal::from_str(&cleaned).map_err(|_| ())?;
    if dec > Decimal::ZERO {
        Ok(dec)
    } else {
        Err(())
    }
}
