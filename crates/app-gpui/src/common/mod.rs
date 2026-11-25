//! Reusable UI primitives shared across screens and feature components
//! (inputs, radios, text areas, cards, etc.).

pub mod accordion;
pub mod alert;
pub mod aspect_ratio;
pub mod avatar;
pub mod badge;
pub mod breadcrumb;
pub mod button;
pub mod button_group;
pub mod card;
pub mod checkbox;
pub mod collapsible;
pub mod direction;
pub mod empty;
pub mod field;
pub mod hover_card;
pub mod input;
pub mod input_group;
pub mod input_otp;
pub mod item;
pub mod kbd;
pub mod label;
pub mod marker;
pub mod native_select;
pub mod pagination;
pub mod popover;
pub mod progress;
pub mod radio;
pub mod radio_group;
pub mod resizable;
pub mod scroll_area;
pub mod separator;
pub mod skeleton;
pub mod slider;
pub mod spinner;
pub mod switch;
pub mod table;
pub mod tabs;
pub mod textarea;
pub mod toggle;
pub mod toggle_group;
pub mod tooltip;
pub mod utils;

pub use accordion::{
    Accordion, AccordionContent, AccordionItem, AccordionMode, AccordionTrigger, KitAccordion,
    KitAccordionItem,
};
pub use alert::{Alert, AlertAction, AlertDescription, AlertTitle, AlertVariant, KitAlert};
pub use aspect_ratio::{AspectRatio, aspect_ratio};
pub use avatar::{
    Avatar, AvatarBadge, AvatarFallback, AvatarGroup, AvatarGroupCount, AvatarImage, AvatarSize,
    KitAvatar, KitAvatarGroup, initials_from_name,
};
pub use badge::{Badge, BadgeVariant, OverlayCountBadge, badge};
pub use breadcrumb::{
    Breadcrumb, BreadcrumbEllipsis, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage,
    BreadcrumbSeparator, KitBreadcrumb, KitBreadcrumbItem,
};
pub use button::{Button, ButtonSize, ButtonVariant, KitButton, button};
pub use button_group::{
    ButtonGroup, ButtonGroupOrientation, ButtonGroupSeparator, ButtonGroupText, KitButtonGroup,
};
pub use card::{
    Card, CardAction, CardContent, CardDescription, CardFooter, CardHeader, CardSize, CardTitle,
};
pub use checkbox::{Checkbox, CheckedState, KitCheckbox, checkbox_field};
pub use collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger, KitCollapsible};
pub use direction::{Direction, DirectionProvider, LogicalStyledExt, set_direction, use_direction};
pub use empty::{
    Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, EmptyMediaVariant, EmptyTitle,
    KitEmpty,
};
pub use field::{
    Field, FieldContent, FieldDescription, FieldError, FieldGroup, FieldLegend,
    FieldLegendVariant, FieldOrientation, FieldSeparator, FieldSet, FieldTitle, field_slot_label,
};
pub use hover_card::{
    HoverCard, HoverCardContent, HoverCardTrigger, KitHoverCard, KitHoverCardState,
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
pub use item::{
    Item, ItemActions, ItemContent, ItemDescription, ItemFooter, ItemGroup, ItemHeader, ItemMedia,
    ItemMediaVariant, ItemSize, ItemTitle, ItemVariant, item_separator,
};
pub use kbd::{Kbd, KbdGroup};
pub use label::{Label, LabelMode, field_label};
pub use marker::{KitMarker, Marker, MarkerContent, MarkerIcon, MarkerVariant};
pub use native_select::{
    NativeSelect, NativeSelectOptGroup, NativeSelectOption, NativeSelectSize,
};
pub use pagination::{
    KitPagination, Pagination, PaginationContent, PaginationEllipsis, PaginationItem,
    PaginationLink, PaginationNext, PaginationPrevious,
};
pub use popover::{
    KitPopover, KitPopoverState, Popover, PopoverAlign, PopoverContent, PopoverDescription,
    PopoverHeader, PopoverSide, PopoverTitle, PopoverTrigger,
};
pub use progress::{
    KitProgress, Progress, ProgressCircle, ProgressIndicator, ProgressLabel, ProgressTrack,
    ProgressValue,
};
pub use radio::{Radio, RadioGroup, radio_button, radio_field, radio_group_field, radio_option};
pub use radio_group::{
    RadioGroup as RadioGroupContainer, RadioGroupItem, RadioGroupOrientation,
};
pub use resizable::{
    KitResizablePanel, KitResizablePanelGroup, ResizableHandle, ResizableOrientation,
    ResizablePanel, ResizablePanelEvent, ResizablePanelGroup, ResizableState,
    kit_h_resizable, kit_resizable_panel, kit_v_resizable, preset_resize_handle_appearance,
};
pub use scroll_area::{
    KitScrollable, KitScrollbar, ScrollArea, ScrollBar, ScrollBarOrientation, ScrollableElement,
    ScrollbarAxis, ScrollbarHandle, ScrollbarMode,
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
pub use table::{
    KitColumnSort, KitDataTable, KitTable, KitTableColumn, KitTableDelegate, KitTableState, Table,
    TableAlign, TableBody, TableCaption, TableCell, TableFooter, TableHead, TableHeader, TableRow,
};
pub use tabs::{
    KitTab, KitTabBar, Tabs, TabsContent, TabsList, TabsListVariant, TabsOrientation, TabsTrigger,
};
pub use textarea::{KitTextarea, Textarea, TextareaState, textarea_field};
pub use toggle::{KitToggle, Toggle, ToggleSize, ToggleVariant};
pub use toggle_group::{ToggleGroup, ToggleGroupItem, ToggleGroupOrientation};
pub use tooltip::{
    KitTooltip, Tooltip, TooltipAlign, TooltipContent, TooltipDefaults, TooltipProvider,
    TooltipSide, TooltipTrigger,
};
pub use utils::{
    PresetRadius, PresetSpacing, PresetTokens, StyleComposeExt, card_surface_shadow,
    focus_ring_shadow, invalid_ring_shadow, overlay_surface_shadow, to_preset_uppercase,
};
