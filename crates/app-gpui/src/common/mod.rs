//! Reusable UI primitives shared across screens and feature components
//! (inputs, radios, text areas, cards, etc.).

pub mod aspect_ratio;
pub mod badge;
pub mod button;
pub mod button_group;
pub mod card;
pub mod checkbox;
pub mod direction;
pub mod input;
pub mod input_group;
pub mod input_otp;
pub mod kbd;
pub mod label;
pub mod radio;
pub mod separator;
pub mod skeleton;
pub mod spinner;
pub mod switch;
pub mod textarea;
pub mod utils;

pub use aspect_ratio::{AspectRatio, aspect_ratio};
pub use badge::{Badge, BadgeVariant, OverlayCountBadge, badge};
pub use button::{Button, ButtonSize, ButtonVariant, KitButton, button};
pub use button_group::{
    ButtonGroup, ButtonGroupOrientation, ButtonGroupSeparator, ButtonGroupText, KitButtonGroup,
};
pub use card::{
    Card, CardAction, CardContent, CardDescription, CardFooter, CardHeader, CardSize, CardTitle,
};
pub use checkbox::{Checkbox, CheckedState, KitCheckbox, checkbox_field};
pub use direction::{Direction, DirectionProvider, LogicalStyledExt, set_direction, use_direction};
pub use input::{
    Input, InputContentType, InputState, KitInput, input_primitive, password_field, text_field,
};
pub use input_group::{
    InputGroup, InputGroupAddon, InputGroupAddonAlign, InputGroupButtonSize, InputGroupText,
    input_group_button, input_group_input, input_group_textarea,
};
pub use input_otp::{
    InputOTP, InputOTPGroup, InputOTPSeparator, InputOTPSlot, KitOtpInput, OtpEvent, OtpState,
};
pub use kbd::{Kbd, KbdGroup};
pub use label::{Label, LabelMode, field_label};
pub use radio::{Radio, RadioGroup, radio_button, radio_field, radio_group_field, radio_option};
pub use separator::{
    Separator, SeparatorOrientation, SeparatorStyle, horizontal_separator, vertical_separator,
};
pub use skeleton::{Skeleton, skeleton};
pub use spinner::{Spinner, spinner, spinner_static};
pub use switch::{KitSwitch, Switch, SwitchSize, switch_field};
pub use textarea::{KitTextarea, Textarea, TextareaState, textarea_field};
pub use utils::{
    PresetRadius, PresetSpacing, PresetTokens, StyleComposeExt, card_surface_shadow,
    focus_ring_shadow, invalid_ring_shadow, overlay_surface_shadow, to_preset_uppercase,
};
