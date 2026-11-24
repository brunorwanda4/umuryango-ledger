//! Common radio button and radio group inputs built on the gpui-kit `Radio`
//! and `RadioGroup` components (<https://gpui-kit.com/component/radio/>).
//!
//! All controls are controlled values: pass `selected_index` (or `checked` for
//! a standalone radio) and update state plus `cx.notify()` inside `on_change`.
//! Checked indicator fill, border, focus ring, and checkmark colors come from
//! the kit theme, which `SemanticTheme::apply_to_kit` keeps in sync with the
//! app palette.

pub use gpui_kit::component::radio::{Radio, RadioGroup};
use gpui_kit::{
    App, Axis, ElementId, FontWeight, IntoElement, ParentElement, SharedString, Styled, Window,
    div, px,
};

use crate::theme::SemanticTheme;

/// Builds a single styled `Radio` option suitable for passing into
/// [`radio_field`] or [`radio_group_field`].
///
/// Callers can chain `.disabled(true)`, `.child(...)` for descriptive text,
/// `.tab_index(...)`, or `Sizable` methods (`.xsmall()`, `.small()`, `.large()`)
/// on the returned [`Radio`].
pub fn radio_option(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Radio {
    Radio::new(id)
        .label(label.into())
        .text_sm()
        .cursor_pointer()
}

/// Standalone controlled radio button with a label and `on_change` callback.
pub fn radio_button(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    checked: bool,
    on_change: impl Fn(&bool, &mut Window, &mut App) + 'static,
) -> Radio {
    radio_option(id, label)
        .checked(checked)
        .on_change(on_change)
}

/// Labeled horizontal radio group for compact mutually exclusive choices
/// (e.g. gender, language, or 2-4 option form selectors).
pub fn radio_field(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    options: impl IntoIterator<Item = impl Into<Radio>>,
    selected_index: Option<usize>,
    on_change: impl Fn(&usize, &mut Window, &mut App) + 'static,
    theme: &SemanticTheme,
) -> impl IntoElement {
    radio_group_with_layout(
        id,
        label,
        Axis::Horizontal,
        options,
        selected_index,
        on_change,
        theme,
    )
}

/// Labeled vertical radio group for longer option lists or multi-line
/// descriptive `Radio` options.
pub fn radio_group_field(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    options: impl IntoIterator<Item = impl Into<Radio>>,
    selected_index: Option<usize>,
    on_change: impl Fn(&usize, &mut Window, &mut App) + 'static,
    theme: &SemanticTheme,
) -> impl IntoElement {
    radio_group_with_layout(
        id,
        label,
        Axis::Vertical,
        options,
        selected_index,
        on_change,
        theme,
    )
}

fn radio_group_with_layout(
    id: impl Into<ElementId>,
    label: impl Into<SharedString>,
    layout: Axis,
    options: impl IntoIterator<Item = impl Into<Radio>>,
    selected_index: Option<usize>,
    on_change: impl Fn(&usize, &mut Window, &mut App) + 'static,
    theme: &SemanticTheme,
) -> impl IntoElement {
    let group = match layout {
        Axis::Horizontal => RadioGroup::horizontal(id),
        Axis::Vertical => RadioGroup::vertical(id),
    }
    .flex_none()
    .w_full()
    .text_sm()
    .text_color(theme.text_primary)
    .children(options)
    .selected_index(selected_index)
    .on_change(on_change);

    div()
        .w_full()
        .flex()
        .flex_col()
        .gap(px(4.0))
        .child(
            div()
                .text_xs()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(theme.text_muted)
                .child(label.into()),
        )
        .child(group)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn radio_option_builds_element() {
        let _option = radio_option("test-option", "Option 1").disabled(false);
        let _standalone = radio_button("test-radio", "Select me", true, |_, _, _| {});
    }
}
