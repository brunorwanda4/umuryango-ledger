//! Reusable UI primitives shared across screens and feature components
//! (inputs, radios, text areas, cards, etc.).

pub mod alert;
pub mod aspect_ratio;
pub mod avatar;
pub mod badge;
pub mod button;
pub mod button_group;
pub mod card;
pub mod checkbox;
pub mod direction;
pub mod field;
pub mod input;
pub mod input_group;
pub mod input_otp;
pub mod kbd;
pub mod label;
pub mod native_select;
pub mod progress;
pub mod radio;
pub mod radio_group;
pub mod separator;
pub mod skeleton;
pub mod slider;
pub mod spinner;
pub mod switch;
pub mod textarea;
pub mod utils;

pub use alert::{Alert, AlertAction, AlertDescription, AlertTitle, AlertVariant, KitAlert};
pub use aspect_ratio::{AspectRatio, aspect_ratio};
pub use avatar::{
    Avatar, AvatarBadge, AvatarFallback, AvatarGroup, AvatarGroupCount, AvatarImage, AvatarSize,
    KitAvatar, KitAvatarGroup, initials_from_name,
};
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
pub use field::{
    Field, FieldContent, FieldDescription, FieldError, FieldGroup, FieldLegend,
    FieldLegendVariant, FieldOrientation, FieldSeparator, FieldSet, FieldTitle, field_slot_label,
};
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
pub use native_select::{
    NativeSelect, NativeSelectOptGroup, NativeSelectOption, NativeSelectSize,
};
pub use progress::{
    KitProgress, Progress, ProgressCircle, ProgressIndicator, ProgressLabel, ProgressTrack,
    ProgressValue,
};
pub use radio::{Radio, RadioGroup, radio_button, radio_field, radio_group_field, radio_option};
pub use radio_group::{
    RadioGroup as RadioGroupContainer, RadioGroupItem, RadioGroupOrientation,
};
pub use separator::{
    Separator, SeparatorOrientation, SeparatorStyle, horizontal_separator, vertical_separator,
};
pub use skeleton::{Skeleton, skeleton};
pub use slider::{
    KitSlider, Slider, SliderEvent, SliderOrientation, SliderScale, SliderState, SliderValue,
    slider_field,
};
pub use spinner::{Spinner, spinner, spinner_static};
pub use switch::{KitSwitch, Switch, SwitchSize, switch_field};
pub use textarea::{KitTextarea, Textarea, TextareaState, textarea_field};
pub use utils::{
    PresetRadius, PresetSpacing, PresetTokens, StyleComposeExt, card_surface_shadow,
    focus_ring_shadow, invalid_ring_shadow, overlay_surface_shadow, to_preset_uppercase,
};
