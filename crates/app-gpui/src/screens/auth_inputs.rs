//! Editable input entities for the Login / Register forms.
//!
//! Every keystroke is mirrored into `AppState` (`auth_*` fields) so the submit
//! handlers keep reading plain strings, and Enter in any field submits the
//! active form.

use gpui_kit::component::input::{InputEvent, InputState};
use gpui_kit::{AppContext, Context, Entity, Subscription, Window};

use crate::RootView;
use crate::state::AppState;

const PHONE_PLACEHOLDER: &str = "+250 788 000 000";
const NAME_PLACEHOLDER: &str = "Kwizera Aimable";

pub struct AuthInputs {
    pub full_name: Entity<InputState>,
    pub phone: Entity<InputState>,
    pub password: Entity<InputState>,
    pub confirm_password: Entity<InputState>,
    _subscriptions: Vec<Subscription>,
}

impl AuthInputs {
    pub fn new(window: &mut Window, cx: &mut Context<RootView>) -> Self {
        let full_name = cx.new(|cx| InputState::new(window, cx).placeholder(NAME_PLACEHOLDER));
        let phone = cx.new(|cx| InputState::new(window, cx).placeholder(PHONE_PLACEHOLDER));
        let password = cx.new(|cx| InputState::new(window, cx).masked(true));
        let confirm_password = cx.new(|cx| InputState::new(window, cx).masked(true));

        let subscriptions = vec![
            bind(&full_name, |s, v| s.auth_full_name = v, cx),
            bind(&phone, |s, v| s.auth_phone = v, cx),
            bind(&password, |s, v| s.auth_password = v, cx),
            bind(&confirm_password, |s, v| s.auth_confirm_password = v, cx),
        ];

        Self {
            full_name,
            phone,
            password,
            confirm_password,
            _subscriptions: subscriptions,
        }
    }
}

/// Mirrors an input's value into `AppState`, refreshes focus borders, and submits on Enter.
fn bind(
    input: &Entity<InputState>,
    apply: fn(&mut AppState, String),
    cx: &mut Context<RootView>,
) -> Subscription {
    cx.subscribe(
        input,
        move |this, input, event: &InputEvent, cx| match event {
            InputEvent::Change => {
                let value = input.read(cx).value().to_string();
                apply(&mut this.state, value);
            }
            InputEvent::PressEnter { .. } => {
                if this.state.auth_mode_register {
                    this.handle_register_submit(cx);
                } else {
                    this.handle_login_submit(cx);
                }
            }
            InputEvent::Focus | InputEvent::Blur => {
                cx.notify();
            }
        },
    )
}
