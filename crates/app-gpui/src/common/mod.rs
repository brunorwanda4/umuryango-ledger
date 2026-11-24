//! Reusable UI primitives shared across screens and feature components
//! (inputs, radios, text areas, cards, etc.).

pub mod aspect_ratio;
pub mod badge;
pub mod button;
pub mod direction;
pub mod input;
pub mod kbd;
pub mod label;
pub mod radio;
pub mod separator;
pub mod skeleton;
pub mod spinner;
pub mod utils;

pub use aspect_ratio::{AspectRatio, aspect_ratio};
pub use badge::{Badge, BadgeVariant, OverlayCountBadge, badge};
pub use button::{Button, ButtonSize, ButtonVariant, KitButton, button};
pub use direction::{Direction, DirectionProvider, LogicalStyledExt, set_direction, use_direction};
pub use input::{password_field, text_field};
pub use kbd::{Kbd, KbdGroup};
pub use label::{Label, LabelMode, field_label};
pub use radio::{Radio, RadioGroup, radio_button, radio_field, radio_group_field, radio_option};
pub use separator::{
    Separator, SeparatorOrientation, SeparatorStyle, horizontal_separator, vertical_separator,
};
pub use skeleton::{Skeleton, skeleton};
pub use spinner::{Spinner, spinner, spinner_static};
pub use utils::{
    PresetRadius, PresetSpacing, PresetTokens, StyleComposeExt, card_surface_shadow,
    focus_ring_shadow, invalid_ring_shadow, overlay_surface_shadow, to_preset_uppercase,
};
