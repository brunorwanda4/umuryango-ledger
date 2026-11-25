//! Editable input entities for the Family Onboarding flow.

use gpui_kit::component::input::{InputEvent, InputState};
use gpui_kit::{AppContext, Context, Entity, Subscription, Window};
use umuryango_i18n::Locale;

use crate::RootView;
use crate::common::{OtpEvent, OtpState};
use crate::state::AppState;

pub const FAMILY_INVITE_CODE_LEN: usize = 4;

pub struct OnboardingInputs {
    pub family_name: Entity<InputState>,
    pub member_phone: Entity<InputState>,
    pub join_code: Entity<OtpState>,
    _subscriptions: Vec<Subscription>,
}

impl OnboardingInputs {
    pub fn new(locale: Locale, window: &mut Window, cx: &mut Context<RootView>) -> Self {
        let fam_ph = umuryango_i18n::t(locale, "onboarding.family_placeholder");
        let phone_ph = umuryango_i18n::t(locale, "onboarding.phone_placeholder");

        let family_name = cx.new(|cx| InputState::new(window, cx).placeholder(fam_ph));
        let member_phone = cx.new(|cx| InputState::new(window, cx).placeholder(phone_ph));
        let join_code = cx.new(|cx| OtpState::new(FAMILY_INVITE_CODE_LEN, window, cx));

        let subscriptions = vec![
            bind_field(
                &family_name,
                |s, v| s.onboarding_family_name = v,
                |this, cx| this.handle_create_family_submit(cx),
                cx,
            ),
            cx.subscribe(
                &member_phone,
                |this, input, event: &InputEvent, cx| match event {
                    InputEvent::Change => {
                        this.state.onboarding_phone_draft = input.read(cx).value().to_string();
                    }
                    InputEvent::PressEnter { .. } => {
                        this.add_onboarding_phone(cx);
                    }
                    InputEvent::Focus | InputEvent::Blur => cx.notify(),
                },
            ),
            cx.observe(&join_code, |_, _, cx| cx.notify()),
            cx.subscribe(&join_code, |this, otp, event: &OtpEvent, cx| match event {
                OtpEvent::Change | OtpEvent::Complete => {
                    this.state.onboarding_join_code = otp.read(cx).value().to_ascii_uppercase();
                    cx.notify();
                }
                OtpEvent::Focus | OtpEvent::Blur => cx.notify(),
            }),
        ];

        Self {
            family_name,
            member_phone,
            join_code,
            _subscriptions: subscriptions,
        }
    }

    pub fn focus_join_code(&self, window: &mut Window, cx: &mut Context<RootView>) {
        self.join_code.update(cx, |otp, cx| otp.focus(window, cx));
    }

    pub fn clear_member_phone(&self, window: &mut Window, cx: &mut Context<RootView>) {
        self.member_phone
            .update(cx, |input, cx| input.set_value("", window, cx));
    }

    pub fn clear_all(&self, window: &mut Window, cx: &mut Context<RootView>) {
        self.family_name
            .update(cx, |input, cx| input.set_value("", window, cx));
        self.member_phone
            .update(cx, |input, cx| input.set_value("", window, cx));
        self.join_code
            .update(cx, |otp, cx| otp.set_value("", window, cx));
    }
}

fn bind_field(
    input: &Entity<InputState>,
    apply: fn(&mut AppState, String),
    on_enter: fn(&mut RootView, &mut Context<RootView>),
    cx: &mut Context<RootView>,
) -> Subscription {
    cx.subscribe(
        input,
        move |this, input, event: &InputEvent, cx| match event {
            InputEvent::Change => {
                let value = input.read(cx).value().to_string();
                apply(&mut this.state, value);
            }
            InputEvent::PressEnter { .. } => on_enter(this, cx),
            InputEvent::Focus | InputEvent::Blur => cx.notify(),
        },
    )
}
