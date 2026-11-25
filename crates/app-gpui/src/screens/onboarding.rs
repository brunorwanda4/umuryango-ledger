use gpui_kit::prelude::*;
use gpui_kit::{Context, ElementId, FontWeight, IntoElement, ParentElement, Styled, div, px};
use umuryango_i18n::{Translator, terms};

use super::onboarding_inputs::OnboardingInputs;
use crate::RootView;
use crate::common::{
    Alert, AlertTitle, Badge, Button, ButtonSize, ButtonVariant, Card, CardContent, CardFooter,
    CardHeader, CardSize, CardTitle, Field, InputOTP, Item, ItemActions, ItemContent, ItemSize,
    Label, Tooltip, text_field,
};
use crate::components::icon::{AppIcon, BrandMark, brand_mark};
use crate::components::lang_toggle::LangToggle;
use crate::state::{AppState, OnboardingStep};
use crate::theme::{SemanticTheme, ThemeMode};

pub struct OnboardingScreen;

impl OnboardingScreen {
    pub fn render(
        id: impl Into<ElementId>,
        state: &AppState,
        inputs: &OnboardingInputs,
        theme: &SemanticTheme,
        cx: &Context<RootView>,
    ) -> impl IntoElement {
        let is_dark = state.theme_mode == ThemeMode::Dark;

        div()
            .id(id)
            .flex_1()
            .h_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .bg(theme.bg_canvas)
            .font_family(theme.font_family)
            .pt(px(16.0) + state.safe_area.top_px())
            .pb(px(16.0) + state.safe_area.total_bottom_px())
            .pl(px(16.0) + state.safe_area.left_px())
            .pr(px(16.0) + state.safe_area.right_px())
            .overflow_y_scroll()
            .relative()
            // Top Bar: Language & Theme Toggles
            .child(
                div()
                    .absolute()
                    .top(px(16.0) + state.safe_area.top_px())
                    .right(px(24.0) + state.safe_area.right_px())
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(10.0))
                    .child(LangToggle::render(
                        "onboarding-lang-toggle",
                        state.locale,
                        theme,
                        cx,
                    ))
                    .child(
                        Button::new("onboarding-theme-toggle")
                            .outline()
                            .size(ButtonSize::IconSm)
                            .icon(if is_dark { AppIcon::Sun } else { AppIcon::Moon })
                            .theme(theme)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.toggle_theme(cx);
                            })),
                    ),
            )
            // Active Onboarding Page
            .child(match state.onboarding_step {
                OnboardingStep::Choose => {
                    Self::render_choose_page(state, theme, cx).into_any_element()
                }
                OnboardingStep::CreateFamily => {
                    Self::render_create_family_page(state, inputs, theme, cx).into_any_element()
                }
                OnboardingStep::JoinFamily => {
                    Self::render_join_family_page(state, inputs, theme, cx).into_any_element()
                }
            })
    }

    fn render_choose_page(
        state: &AppState,
        theme: &SemanticTheme,
        cx: &Context<RootView>,
    ) -> impl IntoElement {
        Card::new()
            .size(CardSize::Sm)
            .w(px(460.0))
            .gap(px(18.0))
            .py(px(24.0))
            .theme(theme)
            .child(
                CardHeader::new()
                    .size(CardSize::Sm)
                    .items_center()
                    .theme(theme)
                    .child(
                        div()
                            .w_full()
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap(px(10.0))
                            .child(brand_mark(BrandMark::Logo, 64.0))
                            .child(
                                CardTitle::new(Translator::t(
                                    state.locale,
                                    terms::ONBOARDING_TITLE,
                                    None,
                                ))
                                .text_center()
                                .theme(theme),
                            ),
                    ),
            )
            .child(
                CardContent::new()
                    .size(CardSize::Sm)
                    .flex()
                    .flex_col()
                    .gap(px(12.0))
                    .children(if !state.pending_invites.is_empty() {
                        Some(Self::render_pending_invites(state, theme, cx))
                    } else {
                        None
                    })
                    .child(
                        Button::new("onboarding-choose-create-btn")
                            .icon_start(AppIcon::Add)
                            .disabled(state.is_busy)
                            .label(Translator::t(state.locale, terms::CREATE_FAMILY, None))
                            .w_full()
                            .theme(theme)
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.set_onboarding_step(OnboardingStep::CreateFamily, window, cx);
                            })),
                    )
                    .child(
                        Button::new("onboarding-choose-join-btn")
                            .outline()
                            .icon_start(AppIcon::FamilyLedger)
                            .disabled(state.is_busy)
                            .label(Translator::t(state.locale, terms::JOIN_FAMILY, None))
                            .w_full()
                            .theme(theme)
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.set_onboarding_step(OnboardingStep::JoinFamily, window, cx);
                            })),
                    ),
            )
            .child(
                CardFooter::new()
                    .size(CardSize::Sm)
                    .justify_center()
                    .theme(theme)
                    .child(
                        Button::new("onboarding-skip-btn")
                            .link()
                            .size(ButtonSize::Sm)
                            .disabled(state.is_busy)
                            .label(Translator::t(state.locale, terms::SKIP_FOR_NOW, None))
                            .theme(theme)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.handle_skip_onboarding(cx);
                            })),
                    ),
            )
    }

    fn render_pending_invites(
        state: &AppState,
        theme: &SemanticTheme,
        cx: &Context<RootView>,
    ) -> impl IntoElement {
        Alert::new()
            .icon(AppIcon::FamilyLedger)
            .bg(theme.accent_primary.opacity(0.08))
            .border_color(theme.accent_primary.opacity(0.4))
            .theme(theme)
            .child(
                AlertTitle::new(Translator::t(
                    state.locale,
                    terms::PENDING_FAMILY_INVITES,
                    None,
                ))
                .text_color(theme.accent_primary),
            )
            .child(
                div().flex().flex_col().gap(px(8.0)).mt(px(4.0)).children(
                    state
                        .pending_invites
                        .iter()
                        .enumerate()
                        .map(|(idx, invite)| {
                            let invite_id = invite.id;
                            let family_name = invite.family_name.clone();
                            let invited_by = umuryango_i18n::t_args(
                                state.locale,
                                "onboarding.invited_by",
                                &[("name", &invite.invited_by_name)],
                            );

                            Item::new()
                                .id(format!("invite-{}", idx))
                                .outline()
                                .size(ItemSize::Xs)
                                .bg(theme.bg_surface)
                                .theme(theme)
                                .child(
                                    ItemContent::new()
                                        .gap(px(2.0))
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(theme.text_primary)
                                                .child(format!(
                                                    "{}: {}",
                                                    Translator::t(
                                                        state.locale,
                                                        terms::YOU_WERE_ADDED_TO,
                                                        None
                                                    ),
                                                    family_name
                                                )),
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(theme.text_muted)
                                                .child(invited_by),
                                        ),
                                )
                                .child(
                                    ItemActions::new().child(
                                        Button::new(format!("accept-invite-btn-{}", idx))
                                            .size(ButtonSize::Xs)
                                            .label(Translator::t(
                                                state.locale,
                                                terms::JOIN_BTN,
                                                None,
                                            ))
                                            .bg(theme.income_color)
                                            .theme(theme)
                                            .on_click(cx.listener(move |this, _, _, cx| {
                                                this.handle_accept_invite(invite_id, cx);
                                            })),
                                    ),
                                )
                        }),
                ),
            )
    }

    fn render_create_family_page(
        state: &AppState,
        inputs: &OnboardingInputs,
        theme: &SemanticTheme,
        cx: &Context<RootView>,
    ) -> impl IntoElement {
        let submit_label = if state.is_busy {
            umuryango_i18n::t(state.locale, "onboarding.creating_family")
        } else {
            Translator::t(state.locale, terms::CONTINUE_TO_DASHBOARD, None)
        };

        Card::new()
            .size(CardSize::Sm)
            .w(px(460.0))
            .gap(px(18.0))
            .py(px(24.0))
            .theme(theme)
            .child(Self::render_step_header(
                "onboarding-create-back-btn",
                Translator::t(state.locale, terms::CREATE_FAMILY, None),
                state,
                theme,
                cx,
            ))
            .child(
                CardContent::new()
                    .size(CardSize::Sm)
                    .flex()
                    .flex_col()
                    .gap(px(14.0))
                    // Editable Family Name Input
                    .child(text_field(
                        "onboarding-family-name-input",
                        Translator::t(state.locale, terms::FAMILY_NAME, None),
                        &inputs.family_name,
                        theme,
                    ))
                    // Editable Member Phone Input + Add Button + Chips
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_end()
                                    .gap(px(8.0))
                                    .child(div().flex_1().child(text_field(
                                        "onboarding-member-phone-input",
                                        Translator::t(state.locale, terms::ADD_MEMBER_PHONE, None),
                                        &inputs.member_phone,
                                        theme,
                                    )))
                                    .child(
                                        Tooltip::new(
                                            "Add-new member",
                                            Translator::t(
                                                state.locale,
                                                terms::ADD_MEMBER_PHONE,
                                                None,
                                            ),
                                        )
                                        .child(
                                            Button::new("add-member-phone-btn")
                                                .outline()
                                                .icon(AppIcon::Add)
                                                .size(ButtonSize::Sm)
                                                .variant(ButtonVariant::Outline)
                                                .disabled(state.is_busy)
                                                .theme(theme)
                                                .on_click(cx.listener(|this, _, window, cx| {
                                                    this.add_onboarding_phone_with_window(
                                                        window, cx,
                                                    );
                                                })),
                                        ),
                                    ),
                            )
                            .children(if !state.onboarding_member_phones.is_empty() {
                                Some(div().flex().flex_wrap().gap(px(6.0)).children(
                                    state.onboarding_member_phones.iter().enumerate().map(
                                        |(idx, phone)| {
                                            Badge::new(phone.clone())
                                                .id(format!("phone-chip-rm-{}", idx))
                                                .outline()
                                                .icon_end(AppIcon::Close)
                                                .px(px(10.0))
                                                .py(px(4.0))
                                                .bg(theme.bg_canvas)
                                                .theme(theme)
                                                .on_click(cx.listener(move |this, _, _, cx| {
                                                    this.remove_onboarding_phone(idx, cx);
                                                }))
                                        },
                                    ),
                                ))
                            } else {
                                None
                            }),
                    )
                    // Create Family Submit Button
                    .child(
                        Button::new("create-family-submit-btn")
                            .loading(state.is_busy)
                            .label(submit_label)
                            .w_full()
                            .theme(theme)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.handle_create_family_submit(cx);
                            })),
                    ),
            )
            .child(
                CardFooter::new()
                    .size(CardSize::Sm)
                    .justify_center()
                    .theme(theme)
                    .child(
                        Button::new("onboarding-switch-to-join-btn")
                            .link()
                            .size(ButtonSize::Sm)
                            .disabled(state.is_busy)
                            .label(Translator::t(state.locale, terms::JOIN_FAMILY, None))
                            .theme(theme)
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.set_onboarding_step(OnboardingStep::JoinFamily, window, cx);
                            })),
                    ),
            )
    }

    fn render_join_family_page(
        state: &AppState,
        inputs: &OnboardingInputs,
        theme: &SemanticTheme,
        cx: &Context<RootView>,
    ) -> impl IntoElement {
        let submit_label = if state.is_busy {
            umuryango_i18n::t(state.locale, "onboarding.joining_family")
        } else {
            Translator::t(state.locale, terms::JOIN_BTN, None)
        };

        Card::new()
            .size(CardSize::Sm)
            .w(px(460.0))
            .gap(px(18.0))
            .py(px(24.0))
            .theme(theme)
            .child(Self::render_step_header(
                "onboarding-join-back-btn",
                Translator::t(state.locale, terms::JOIN_FAMILY, None),
                state,
                theme,
                cx,
            ))
            .child(
                CardContent::new()
                    .size(CardSize::Sm)
                    .flex()
                    .flex_col()
                    .gap(px(20.0))
                    // 4-Character Uppercase OTP Input
                    .child(
                        Field::new()
                            .items_center()
                            .gap(px(12.0))
                            .theme(theme)
                            .child(
                                Label::new(Translator::t(
                                    state.locale,
                                    terms::ENTER_4_CHAR_CODE,
                                    None,
                                ))
                                .text_center()
                                .theme(theme),
                            )
                            .child(
                                InputOTP::new(&inputs.join_code)
                                    .groups(1)
                                    .uppercase_alphanumeric(true)
                                    .disabled(state.is_busy)
                                    .theme(theme)
                                    .on_change(cx.listener(|this, code: &String, _, cx| {
                                        this.state.onboarding_join_code = code.clone();
                                        cx.notify();
                                    }))
                                    .on_submit(cx.listener(|this, code: &String, _, cx| {
                                        this.state.onboarding_join_code = code.clone();
                                        this.handle_join_family_submit(cx);
                                    })),
                            ),
                    )
                    // Join Family Submit Button
                    .child(
                        Button::new("join-family-submit-btn")
                            .loading(state.is_busy)
                            .label(submit_label)
                            .w_full()
                            .theme(theme)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.handle_join_family_submit(cx);
                            })),
                    ),
            )
            .child(
                CardFooter::new()
                    .size(CardSize::Sm)
                    .justify_center()
                    .theme(theme)
                    .child(
                        Button::new("onboarding-switch-to-create-btn")
                            .link()
                            .size(ButtonSize::Sm)
                            .disabled(state.is_busy)
                            .label(Translator::t(state.locale, terms::CREATE_FAMILY, None))
                            .theme(theme)
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.set_onboarding_step(OnboardingStep::CreateFamily, window, cx);
                            })),
                    ),
            )
    }

    fn render_step_header(
        back_btn_id: &'static str,
        title: String,
        state: &AppState,
        theme: &SemanticTheme,
        cx: &Context<RootView>,
    ) -> impl IntoElement {
        CardHeader::new()
            .size(CardSize::Sm)
            .items_center()
            .theme(theme)
            .child(
                div()
                    .w_full()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap(px(10.0))
                    .child(
                        div()
                            .w_full()
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_start()
                            .child(
                                Button::new(back_btn_id)
                                    .ghost()
                                    .size(ButtonSize::Xs)
                                    .icon_start(AppIcon::ArrowLeft)
                                    .disabled(state.is_busy)
                                    .label(umuryango_i18n::t(state.locale, "common.back"))
                                    .theme(theme)
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.set_onboarding_step(
                                            OnboardingStep::Choose,
                                            window,
                                            cx,
                                        );
                                    })),
                            ),
                    )
                    .child(brand_mark(BrandMark::Logo, 56.0))
                    .child(CardTitle::new(title).text_center().theme(theme)),
            )
    }
}
