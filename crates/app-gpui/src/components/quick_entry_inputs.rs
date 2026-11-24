//! Editable input entities for the Quick Entry Transaction dialog.

use gpui_kit::component::input::{InputEvent, InputState};
use gpui_kit::{AppContext, Context, Entity, Subscription, Window};

use crate::RootView;

pub struct QuickEntryInputs {
    pub amount: Entity<InputState>,
    pub description: Entity<InputState>,
    pub paid_to: Entity<InputState>,
    _subscriptions: Vec<Subscription>,
}

impl QuickEntryInputs {
    pub fn new(window: &mut Window, cx: &mut Context<RootView>) -> Self {
        let amount = cx.new(|cx| InputState::new(window, cx).placeholder("10000"));
        let description = cx.new(|cx| InputState::new(window, cx).placeholder("..."));
        let paid_to = cx.new(|cx| InputState::new(window, cx).placeholder("..."));

        let subscriptions = vec![
            cx.subscribe(&amount, |this, input, event: &InputEvent, cx| match event {
                InputEvent::Change => {
                    this.quick_entry_form.amount_input = input.read(cx).value().to_string();
                }
                InputEvent::PressEnter { .. } => this.submit_quick_entry(cx),
                InputEvent::Focus | InputEvent::Blur => cx.notify(),
            }),
            cx.subscribe(
                &description,
                |this, input, event: &InputEvent, cx| match event {
                    InputEvent::Change => {
                        this.quick_entry_form.description = input.read(cx).value().to_string();
                    }
                    InputEvent::PressEnter { .. } => this.submit_quick_entry(cx),
                    InputEvent::Focus | InputEvent::Blur => cx.notify(),
                },
            ),
            cx.subscribe(
                &paid_to,
                |this, input, event: &InputEvent, cx| match event {
                    InputEvent::Change => {
                        this.quick_entry_form.paid_to = input.read(cx).value().to_string();
                    }
                    InputEvent::PressEnter { .. } => this.submit_quick_entry(cx),
                    InputEvent::Focus | InputEvent::Blur => cx.notify(),
                },
            ),
        ];

        Self {
            amount,
            description,
            paid_to,
            _subscriptions: subscriptions,
        }
    }

    pub fn set_amount(&self, val: &str, window: &mut Window, cx: &mut Context<RootView>) {
        self.amount
            .update(cx, |input, cx| input.set_value(val, window, cx));
    }

    pub fn reset(&self, window: &mut Window, cx: &mut Context<RootView>) {
        self.amount
            .update(cx, |input, cx| input.set_value("", window, cx));
        self.description
            .update(cx, |input, cx| input.set_value("", window, cx));
        self.paid_to
            .update(cx, |input, cx| input.set_value("", window, cx));
    }
}
