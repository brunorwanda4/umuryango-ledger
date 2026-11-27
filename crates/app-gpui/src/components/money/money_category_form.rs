//! Money category creation dialog (`Income` vs `Expense` radio + category name input)
//! composed from `common` primitives (`AlertDialog`, `RadioGroupContainer`, `text_field`, `Button`)
//! and persisted to both local SQLite (`money_categories` + `sync_queue`) and PostgreSQL.

use chrono::Utc;
use gpui_kit::component::input::{InputEvent, InputState};
use gpui_kit::{
    AppContext, Context, ElementId, Entity, InteractiveElement, IntoElement, ParentElement, Styled,
    Subscription, Window, div, px,
};
use umuryango_core_domain::models::{CategoryKind, MoneyCategory};
use umuryango_i18n::{Locale, t, t_args};
use uuid::Uuid;

#[cfg(not(target_arch = "wasm32"))]
use umuryango_api_client::CreateCategoryRequest;

use crate::RootView;
use crate::common::{
    AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent, AlertDialogDescription,
    AlertDialogFooter, AlertDialogHeader, AlertDialogTitle, AlertDialogTrigger, Button, ButtonSize,
    Field, Label, RadioGroupContainer, RadioGroupItem, text_field,
};
use crate::components::icon::AppIcon;
use crate::components::quick_entry::QuickEntryMode;
use crate::state::AppState;
use crate::theme::SemanticTheme;

/// Managed text input entity for [`MoneyCategoryForm`].
pub struct MoneyCategoryInputs {
    pub name: Entity<InputState>,
    _subscription: Subscription,
}

impl MoneyCategoryInputs {
    pub fn new(locale: Locale, window: &mut Window, cx: &mut Context<RootView>) -> Self {
        let placeholder = t(locale, "categories.name_placeholder");
        let name = cx.new(|cx| InputState::new(window, cx).placeholder(placeholder));

        let subscription = cx.subscribe(&name, |this, input, event: &InputEvent, cx| match event {
            InputEvent::Change => {
                this.state.category_form_name = input.read(cx).value().to_string();
            }
            InputEvent::PressEnter { .. } => {
                this.submit_money_category(cx);
            }
            InputEvent::Focus | InputEvent::Blur => cx.notify(),
        });

        Self {
            name,
            _subscription: subscription,
        }
    }

    pub fn reset_and_focus(&self, locale: Locale, window: &mut Window, cx: &mut Context<RootView>) {
        let placeholder = t(locale, "categories.name_placeholder");
        self.name.update(cx, |input, cx| {
            input.set_placeholder(placeholder, window, cx);
            input.set_value("", window, cx);
            input.focus(window, cx);
        });
    }

    pub fn clear(&self, window: &mut Window, cx: &mut Context<RootView>) {
        self.name
            .update(cx, |input, cx| input.set_value("", window, cx));
    }
}

pub struct MoneyCategoryForm;

impl MoneyCategoryForm {
    pub fn render(
        id: impl Into<ElementId>,
        default_kind: CategoryKind,
        state: &AppState,
        inputs: &MoneyCategoryInputs,
        theme: &SemanticTheme,
        cx: &Context<RootView>,
    ) -> impl IntoElement {
        let elem_id: ElementId = id.into();
        let dialog_key = format!("{elem_id}");
        let is_open = state.active_category_dialog.as_deref() == Some(dialog_key.as_str());
        let locale = state.locale;

        let selected_radio_idx = match state.category_form_kind {
            CategoryKind::Income => 0usize,
            CategoryKind::Expense => 1usize,
        };

        let view_open = cx.entity().clone();
        let dialog_key_for_open = dialog_key.clone();
        let view_radio = cx.entity().clone();
        let view_cancel = cx.entity().clone();
        let view_save = cx.entity().clone();

        let trigger = AlertDialogTrigger::new(format!("{dialog_key}-trigger")).child(
            Button::new(format!("{dialog_key}-btn"))
                .outline()
                .size(ButtonSize::Sm)
                .icon_start(AppIcon::Add)
                .label(t(locale, "categories.create_btn"))
                .theme(theme),
        );

        let content = AlertDialogContent::new(format!("{dialog_key}-content"))
            .theme(theme)
            .child(
                AlertDialogHeader::new()
                    .child(AlertDialogTitle::new(t(locale, "categories.dialog_title")).theme(theme))
                    .child(
                        AlertDialogDescription::new(t(locale, "categories.dialog_desc"))
                            .theme(theme),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .w_full()
                    .gap(px(16.0))
                    // 1. Radio input: Income or Expense
                    .child(
                        Field::new()
                            .gap(px(8.0))
                            .theme(theme)
                            .child(Label::new(t(locale, "categories.type_label")).theme(theme))
                            .child(
                                RadioGroupContainer::horizontal(format!("{dialog_key}-kind-radio"))
                                    .selected_index(Some(selected_radio_idx))
                                    .item(
                                        RadioGroupItem::new(format!("{dialog_key}-kind-income"))
                                            .label(t(locale, "categories.income"))
                                            .primary_accent(true),
                                    )
                                    .item(
                                        RadioGroupItem::new(format!("{dialog_key}-kind-expense"))
                                            .label(t(locale, "categories.expense"))
                                            .primary_accent(true),
                                    )
                                    .theme(theme)
                                    .on_change(move |idx, _window, cx| {
                                        let kind = if *idx == 0 {
                                            CategoryKind::Income
                                        } else {
                                            CategoryKind::Expense
                                        };
                                        view_radio.update(cx, |this, cx| {
                                            this.state.category_form_kind = kind;
                                            cx.notify();
                                        });
                                    }),
                            ),
                    )
                    // 2. Category Name text input
                    .child(text_field(
                        "money-category-name-input",
                        t(locale, "categories.name_label"),
                        &inputs.name,
                        theme,
                    )),
            )
            .child(
                AlertDialogFooter::new()
                    .child(
                        AlertDialogCancel::new(
                            format!("{dialog_key}-cancel"),
                            t(locale, "common.cancel"),
                        )
                        .theme(theme)
                        .on_click(move |_, window, cx| {
                            view_cancel.update(cx, |this, cx| {
                                this.close_money_category_dialog(Some(window), cx);
                            });
                        }),
                    )
                    .child(
                        AlertDialogAction::new(
                            format!("{dialog_key}-save"),
                            if state.is_busy {
                                t(locale, "common.loading")
                            } else {
                                t(locale, "common.save")
                            },
                        )
                        .loading(state.is_busy)
                        .theme(theme)
                        .on_click(move |_, window, cx| {
                            view_save.update(cx, |this, cx| {
                                this.submit_money_category_with_window(window, cx);
                            });
                        }),
                    ),
            );

        div().id(elem_id.clone()).child(
            AlertDialog::new(format!("{dialog_key}-dialog"))
                .open(is_open)
                .on_open_change(move |next_open, window, cx| {
                    let key = dialog_key_for_open.clone();
                    view_open.update(cx, |this, cx| {
                        if *next_open {
                            this.open_money_category_dialog(key, default_kind, window, cx);
                        } else {
                            this.close_money_category_dialog(Some(window), cx);
                        }
                    });
                })
                .trigger(trigger)
                .content(content),
        )
    }

    /// Validates category inputs and builds a [`MoneyCategory`] ready for database persistence.
    pub fn build_category(state: &AppState, raw_name: &str) -> Result<MoneyCategory, String> {
        let trimmed = raw_name.trim();
        if trimmed.is_empty() {
            return Err(t(state.locale, "categories.name_required"));
        }

        let kind = state.category_form_kind;
        let is_duplicate = state
            .categories
            .iter()
            .any(|c| c.kind == kind && c.name.trim().eq_ignore_ascii_case(trimmed));
        if is_duplicate {
            return Err(t_args(
                state.locale,
                "categories.duplicate_error",
                &[("name", trimmed)],
            ));
        }

        Ok(MoneyCategory {
            id: Uuid::new_v4(),
            scope: state.active_scope,
            kind,
            name: trimmed.to_string(),
            created_at: Utc::now(),
        })
    }
}

impl RootView {
    pub fn open_money_category_dialog(
        &mut self,
        dialog_key: String,
        default_kind: CategoryKind,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.state.active_category_dialog = Some(dialog_key);
        self.state.category_form_kind = default_kind;
        self.state.category_form_name.clear();
        self.money_category_inputs
            .reset_and_focus(self.state.locale, window, cx);
        cx.notify();
    }

    pub fn close_money_category_dialog(
        &mut self,
        window: Option<&mut Window>,
        cx: &mut Context<Self>,
    ) {
        self.state.active_category_dialog = None;
        self.state.category_form_name.clear();
        if let Some(win) = window {
            self.money_category_inputs.clear(win, cx);
        }
        cx.notify();
    }

    pub fn open_quick_entry_for_category(
        &mut self,
        kind: CategoryKind,
        category_name: String,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.open_quick_entry(window, cx);
        self.quick_entry_form.mode = match kind {
            CategoryKind::Income => QuickEntryMode::Income,
            CategoryKind::Expense => QuickEntryMode::Expense,
        };
        self.quick_entry_form.category = category_name;
        cx.notify();
    }

    pub fn submit_money_category_with_window(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.persist_money_category(cx) {
            self.money_category_inputs.clear(window, cx);
        }
    }

    pub fn submit_money_category(&mut self, cx: &mut Context<Self>) {
        let _ = self.persist_money_category(cx);
    }

    fn persist_money_category(&mut self, cx: &mut Context<Self>) -> bool {
        if self.state.is_busy {
            return false;
        }

        let raw_name = self.state.category_form_name.clone();
        let category = match MoneyCategoryForm::build_category(&self.state, &raw_name) {
            Ok(cat) => cat,
            Err(err_msg) => {
                self.state.set_toast(err_msg, true);
                cx.notify();
                return false;
            }
        };

        let locale = self.state.locale;
        let saved_name = category.name.clone();
        let saved_kind = category.kind;
        let has_online_session = self.state.auth_token.is_some();

        // 1. Immediately persist to local SQLite (and queue offline if unauthenticated)
        {
            let db_lock = self.db.as_ref().and_then(|d| d.lock().ok());
            self.state.save_money_category_local(
                category.clone(),
                db_lock.as_deref(),
                !has_online_session,
            );
        }

        // Sync Quick Entry selection to the newly created category
        self.quick_entry_form.category = saved_name.clone();
        if self.state.show_quick_entry && self.quick_entry_form.mode != QuickEntryMode::Handover {
            self.quick_entry_form.mode = match saved_kind {
                CategoryKind::Income => QuickEntryMode::Income,
                CategoryKind::Expense => QuickEntryMode::Expense,
            };
        }

        let toast_msg = t_args(locale, "categories.saved_toast", &[("name", &saved_name)]);

        self.state.active_category_dialog = None;
        self.state.category_form_name.clear();
        self.state.set_toast(toast_msg, false);
        cx.notify();

        // 2. Persist to PostgreSQL backend asynchronously (enqueuing in SyncQueue if offline)
        #[cfg(not(target_arch = "wasm32"))]
        if has_online_session {
            let client = self.api.client.clone();
            let scope = self.state.active_scope;
            let req = CreateCategoryRequest {
                scope: scope.as_str().to_string(),
                family_id: scope.family_id(),
                kind: category.kind,
                name: category.name.clone(),
                idempotency_key: Some(category.id),
            };
            let fut = self
                .api
                .run(async move { client.create_category(&req).await });
            let fallback_cat = category;

            cx.spawn(async move |this, cx| {
                let res = fut.await;
                let _ = this.update(cx, |this, cx| {
                    match res {
                        Ok(remote_cat) => {
                            let db_lock = this.db.as_ref().and_then(|d| d.lock().ok());
                            this.state.save_money_category_local(
                                remote_cat,
                                db_lock.as_deref(),
                                false,
                            );
                        }
                        Err(err) if err.is_network_error() => {
                            let db_lock = this.db.as_ref().and_then(|d| d.lock().ok());
                            this.state.save_money_category_local(
                                fallback_cat,
                                db_lock.as_deref(),
                                true,
                            );
                        }
                        Err(err) => {
                            this.state
                                .set_toast(err.user_message(this.state.locale), true);
                        }
                    }
                    cx.notify();
                });
            })
            .detach();
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use umuryango_core_domain::scope::LedgerScope;

    #[test]
    fn build_category_rejects_empty_and_duplicate_names() {
        let user_id = Uuid::new_v4();
        let mut state = AppState {
            locale: Locale::En,
            user_id,
            active_scope: LedgerScope::Personal { user_id },
            category_form_kind: CategoryKind::Expense,
            ..Default::default()
        };

        assert!(MoneyCategoryForm::build_category(&state, "   ").is_err());

        let cat = MoneyCategoryForm::build_category(&state, "  School Fees  ")
            .expect("valid expense category");
        assert_eq!(cat.name, "School Fees");
        assert_eq!(cat.kind, CategoryKind::Expense);

        state.categories.push(cat);

        // Same kind + case-insensitive duplicate rejected
        assert!(MoneyCategoryForm::build_category(&state, "school fees").is_err());

        // Different kind (Income) with same name allowed
        state.category_form_kind = CategoryKind::Income;
        let income_cat =
            MoneyCategoryForm::build_category(&state, "School Fees").expect("valid income cat");
        assert_eq!(income_cat.kind, CategoryKind::Income);
    }
}
