//! Reusable editable input entities for entity creation/update modals
//! (AddWallet, AddProject, AddLoan, RepayLoan, AddPlan, AllocatePlan, InviteMember).

use gpui_kit::component::input::{InputEvent, InputState};
use gpui_kit::{AppContext, Context, Entity, Subscription, Window};

use crate::RootView;

pub struct EntityModalInputs {
    pub primary: Entity<InputState>,
    pub secondary: Entity<InputState>,
    pub amount: Entity<InputState>,
    _subscriptions: Vec<Subscription>,
}

impl EntityModalInputs {
    pub fn new(window: &mut Window, cx: &mut Context<RootView>) -> Self {
        let primary = cx.new(|cx| InputState::new(window, cx));
        let secondary = cx.new(|cx| InputState::new(window, cx));
        let amount = cx.new(|cx| InputState::new(window, cx).placeholder("0"));

        let subscriptions = vec![
            cx.subscribe(
                &primary,
                |this, input, event: &InputEvent, cx| match event {
                    InputEvent::Change => {
                        this.state.entity_form.primary_text = input.read(cx).value().to_string();
                    }
                    InputEvent::PressEnter { .. } => this.submit_entity_modal(cx),
                    InputEvent::Focus | InputEvent::Blur => cx.notify(),
                },
            ),
            cx.subscribe(
                &secondary,
                |this, input, event: &InputEvent, cx| match event {
                    InputEvent::Change => {
                        this.state.entity_form.secondary_text = input.read(cx).value().to_string();
                    }
                    InputEvent::PressEnter { .. } => this.submit_entity_modal(cx),
                    InputEvent::Focus | InputEvent::Blur => cx.notify(),
                },
            ),
            cx.subscribe(&amount, |this, input, event: &InputEvent, cx| match event {
                InputEvent::Change => {
                    this.state.entity_form.amount_text = input.read(cx).value().to_string();
                }
                InputEvent::PressEnter { .. } => this.submit_entity_modal(cx),
                InputEvent::Focus | InputEvent::Blur => cx.notify(),
            }),
        ];

        Self {
            primary,
            secondary,
            amount,
            _subscriptions: subscriptions,
        }
    }

    pub fn reset(&self, window: &mut Window, cx: &mut Context<RootView>) {
        self.primary
            .update(cx, |input, cx| input.set_value("", window, cx));
        self.secondary
            .update(cx, |input, cx| input.set_value("", window, cx));
        self.amount
            .update(cx, |input, cx| input.set_value("", window, cx));
    }
}
