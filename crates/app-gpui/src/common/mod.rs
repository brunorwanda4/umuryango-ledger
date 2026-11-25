//! Reusable UI primitives shared across screens and feature components
//! (inputs, radios, text areas, cards, etc.).

pub mod accordion;
pub mod alert;
pub mod alert_dialog;
pub mod aspect_ratio;
pub mod attachment;
pub mod avatar;
pub mod badge;
pub mod breadcrumb;
pub mod bubble;
pub mod button;
pub mod button_group;
pub mod calendar;
pub mod card;
pub mod carousel;
pub mod chart;
pub mod checkbox;
pub mod code_block;
pub mod collapsible;
pub mod combobox;
pub mod command;
pub mod context_menu;
pub mod dialog;
pub mod direction;
pub mod drawer;
pub mod dropdown_menu;
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
pub mod menubar;
pub mod message;
pub mod message_scroller;
pub mod native_select;
pub mod navigation_menu;
pub mod pagination;
pub mod popover;
pub mod progress;
pub mod questionnaire;
pub mod radio;
pub mod radio_group;
pub mod resizable;
pub mod scroll_area;
pub mod select;
pub mod separator;
pub mod sheet;
pub mod sidebar;
pub mod skeleton;
pub mod slider;
pub mod spinner;
pub mod switch;
pub mod table;
pub mod tabs;
pub mod textarea;
pub mod toast;
pub mod toggle;
pub mod toggle_group;
pub mod tooltip;
pub mod trace;
pub mod utils;

pub use accordion::{
    Accordion, AccordionContent, AccordionItem, AccordionMode, AccordionTrigger, KitAccordion,
    KitAccordionItem,
};
pub use alert::{Alert, AlertAction, AlertDescription, AlertTitle, AlertVariant, KitAlert};
pub use alert_dialog::{
    AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent, AlertDialogDescription,
    AlertDialogFooter, AlertDialogHeader, AlertDialogMedia, AlertDialogOverlay, AlertDialogPortal,
    AlertDialogSize, AlertDialogTitle, AlertDialogTrigger, KitAlertDialog,
};
pub use aspect_ratio::{AspectRatio, aspect_ratio};
pub use attachment::{
    Attachment, AttachmentAction, AttachmentActions, AttachmentContent, AttachmentDescription,
    AttachmentGroup, AttachmentMedia, AttachmentMediaVariant, AttachmentOrientation,
    AttachmentSize, AttachmentState, AttachmentTitle, AttachmentTrigger, KitAttachment,
    KitAttachmentActions, KitAttachmentContent, KitAttachmentDescription, KitAttachmentGroup,
    KitAttachmentMedia, KitAttachmentStatus, KitAttachmentTitle,
};
pub use avatar::{
    Avatar, AvatarBadge, AvatarFallback, AvatarGroup, AvatarGroupCount, AvatarImage, AvatarSize,
    KitAvatar, KitAvatarGroup, initials_from_name,
};
pub use badge::{Badge, BadgeVariant, OverlayCountBadge, badge};
pub use breadcrumb::{
    Breadcrumb, BreadcrumbEllipsis, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage,
    BreadcrumbSeparator, KitBreadcrumb, KitBreadcrumbItem,
};
pub use bubble::{
    Bubble, BubbleAlign, BubbleContent, BubbleGroup, BubbleReactionSide, BubbleReactions,
    BubbleVariant, KitBubble, KitBubbleContent, KitBubbleGroup, KitBubbleReactionSide,
    KitBubbleReactions, KitBubbleVariant,
};
pub use button::{Button, ButtonSize, ButtonVariant, KitButton, button};
pub use button_group::{
    ButtonGroup, ButtonGroupOrientation, ButtonGroupSeparator, ButtonGroupText, KitButtonGroup,
};
pub use calendar::{
    Calendar, CalendarDayButton, CalendarSelection, KitCalendar, KitCalendarDate, KitCalendarEvent,
    KitCalendarMatcher, KitCalendarState, KitDatePicker, KitDatePickerEvent, KitDatePickerState,
};
pub use card::{
    Card, CardAction, CardContent, CardDescription, CardFooter, CardHeader, CardSize, CardTitle,
};
pub use carousel::{
    Carousel, CarouselContent, CarouselItem, CarouselNext, CarouselOrientation, CarouselPrevious,
    KitCarousel, KitCarouselContent, KitCarouselEvent, KitCarouselItem, KitCarouselNext,
    KitCarouselPagination, KitCarouselPrevious, KitCarouselState,
};
pub use chart::{
    AreaChart, BarChart, CandlestickChart, ChartConfig, ChartContainer, ChartLegend,
    ChartLegendAlign, ChartLegendContent, ChartSeriesConfig, ChartStyle, ChartTooltip,
    ChartTooltipContent, ChartTooltipDatum, ChartTooltipIndicator, LineChart, PieChart, RadarChart,
    RadarLabel, SankeyChart, SankeyLabel, preset_chart_color,
};
pub use checkbox::{Checkbox, CheckedState, KitCheckbox, checkbox_field};
pub use code_block::{
    CodeBlock, CodeBlockActions, CodeBlockContent, CodeBlockCopyButton, CodeBlockFooter,
    CodeBlockHeader, CodeBlockLine, CodeBlockLineVariant, CodeBlockSize, CodeBlockTitle,
    CodeBlockVariant,
};
pub use collapsible::{Collapsible, CollapsibleContent, CollapsibleTrigger, KitCollapsible};
pub use combobox::{
    Combobox, ComboboxChip, ComboboxChips, ComboboxClear, ComboboxCollection, ComboboxContent,
    ComboboxEmpty, ComboboxGroup, ComboboxInput, ComboboxItem, ComboboxLabel, ComboboxList,
    ComboboxSeparator, ComboboxTrigger, ComboboxValue, KitCombobox, KitComboboxChange,
    KitComboboxEvent, KitComboboxState, combobox_chips_input,
};
pub use command::{
    Command, CommandDialog, CommandEmpty, CommandGroup, CommandInput, CommandItem, CommandList,
    CommandSeparator, CommandShortcut, KitCommand, KitCommandEntry, KitCommandGroup,
    KitCommandItem, KitCommandState,
};
pub use context_menu::{
    ContextMenu, ContextMenuCheckboxItem, ContextMenuContent, ContextMenuExt, ContextMenuGroup,
    ContextMenuItem, ContextMenuItemVariant, ContextMenuLabel, ContextMenuPortal,
    ContextMenuRadioGroup, ContextMenuRadioItem, ContextMenuSeparator, ContextMenuShortcut,
    ContextMenuState, ContextMenuSub, ContextMenuSubContent, ContextMenuSubTrigger,
    ContextMenuTrigger, KitContextMenu,
};
pub use dialog::{
    Dialog, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader,
    DialogOverlay, DialogPortal, DialogTitle, DialogTrigger, KitDialog,
};
pub use direction::{Direction, DirectionProvider, LogicalStyledExt, set_direction, use_direction};
pub use drawer::{
    Drawer, DrawerClose, DrawerContent, DrawerDescription, DrawerDirection, DrawerFooter,
    DrawerHeader, DrawerOverlay, DrawerPortal, DrawerSwipeHandle, DrawerTitle, DrawerTrigger,
};
pub use dropdown_menu::{
    DropdownMenu, DropdownMenuCheckboxItem, DropdownMenuContent, DropdownMenuGroup,
    DropdownMenuItem, DropdownMenuItemVariant, DropdownMenuLabel, DropdownMenuPortal,
    DropdownMenuRadioGroup, DropdownMenuRadioItem, DropdownMenuSeparator, DropdownMenuShortcut,
    DropdownMenuSub, DropdownMenuSubContent, DropdownMenuSubTrigger, DropdownMenuTrigger,
    KitDropdownMenu, KitPopupMenu, KitPopupMenuItem,
};
pub use empty::{
    Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, EmptyMediaVariant, EmptyTitle,
    KitEmpty,
};
pub use field::{
    Field, FieldContent, FieldDescription, FieldError, FieldGroup, FieldLegend, FieldLegendVariant,
    FieldOrientation, FieldSeparator, FieldSet, FieldTitle, field_slot_label,
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
pub use menubar::{
    KitAppMenuBar, Menubar, MenubarCheckboxItem, MenubarContent, MenubarGroup, MenubarItem,
    MenubarItemVariant, MenubarLabel, MenubarMenu, MenubarPortal, MenubarRadioGroup,
    MenubarRadioItem, MenubarSeparator, MenubarShortcut, MenubarSub, MenubarSubContent,
    MenubarSubTrigger, MenubarTrigger, menubar_checkbox_item, menubar_content, menubar_radio_item,
};
pub use message::{
    KitMessage, KitMessageAlignment, KitMessageAvatar, KitMessageContent, KitMessageFooter,
    KitMessageGroup, KitMessageHeader, Message, MessageAlign, MessageAvatar, MessageContent,
    MessageFooter, MessageGroup, MessageHeader,
};
pub use message_scroller::{
    KitMessageScroller, KitMessageScrollerState, MessageScroller, MessageScrollerButton,
    MessageScrollerContent, MessageScrollerDirection, MessageScrollerItem, MessageScrollerProvider,
    MessageScrollerViewport, message_scroller_button_visible, message_scroller_is_scrollable,
    scroll_message_scroller_to,
};
pub use native_select::{NativeSelect, NativeSelectOptGroup, NativeSelectOption, NativeSelectSize};
pub use navigation_menu::{
    NavigationMenu, NavigationMenuContent, NavigationMenuIndicator, NavigationMenuItem,
    NavigationMenuLink, NavigationMenuList, NavigationMenuPositioner, NavigationMenuTrigger,
    navigation_menu_trigger_style,
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
pub use questionnaire::{
    KitQuestionnaireAnswer, KitQuestionnaireEvent, KitQuestionnaireState, Questionnaire,
    QuestionnaireActions, QuestionnaireChoice, QuestionnaireChoiceDescription,
    QuestionnaireChoiceType, QuestionnaireChoices, QuestionnaireDescription, QuestionnaireError,
    QuestionnaireInput, QuestionnaireItem, QuestionnaireNext, QuestionnairePrevious,
    QuestionnaireProgress, QuestionnaireSkip, QuestionnaireSubmit, QuestionnaireTitle,
};
pub use radio::{Radio, RadioGroup, radio_button, radio_field, radio_group_field, radio_option};
pub use radio_group::{RadioGroup as RadioGroupContainer, RadioGroupItem, RadioGroupOrientation};
pub use resizable::{
    KitResizablePanel, KitResizablePanelGroup, ResizableHandle, ResizableOrientation,
    ResizablePanel, ResizablePanelEvent, ResizablePanelGroup, ResizableState, kit_h_resizable,
    kit_resizable_panel, kit_v_resizable, preset_resize_handle_appearance,
};
pub use scroll_area::{
    KitScrollable, KitScrollbar, ScrollArea, ScrollBar, ScrollBarOrientation, ScrollableElement,
    ScrollbarAxis, ScrollbarHandle, ScrollbarMode,
};
pub use select::{
    KitSelect, KitSelectDelegate, KitSelectEvent, KitSelectState, Select, SelectContent,
    SelectGroup, SelectItem, SelectLabel, SelectScrollDownButton, SelectScrollUpButton,
    SelectSeparator, SelectTrigger, SelectTriggerSize, SelectValue,
};
pub use separator::{
    Separator, SeparatorOrientation, SeparatorStyle, horizontal_separator, vertical_separator,
};
pub use sheet::{
    KitSheet, Sheet, SheetClose, SheetContent, SheetDescription, SheetFooter, SheetHeader,
    SheetOverlay, SheetPortal, SheetSide, SheetTitle, SheetTrigger,
};
pub use sidebar::{
    KitSidebar, KitSidebarCollapsible, KitSidebarFooter, KitSidebarGroup, KitSidebarHeader,
    KitSidebarItem, KitSidebarMenu, KitSidebarMenuItem, KitSidebarToggleButton,
    SIDEBAR_KEYBOARD_SHORTCUT, SIDEBAR_WIDTH, SIDEBAR_WIDTH_ICON, SIDEBAR_WIDTH_MOBILE, Sidebar,
    SidebarCollapsible, SidebarContent, SidebarContextState, SidebarFooter, SidebarGroup,
    SidebarGroupAction, SidebarGroupContent, SidebarGroupLabel, SidebarHeader, SidebarInput,
    SidebarInset, SidebarMenu, SidebarMenuAction, SidebarMenuBadge, SidebarMenuButton,
    SidebarMenuButtonSize, SidebarMenuButtonVariant, SidebarMenuItem, SidebarMenuSkeleton,
    SidebarMenuSub, SidebarMenuSubButton, SidebarMenuSubButtonSize, SidebarMenuSubItem,
    SidebarProvider, SidebarRail, SidebarSeparator, SidebarSide, SidebarState, SidebarTrigger,
    SidebarVariant, use_sidebar,
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
pub use toast::{
    KitNotification, KitNotificationList, KitNotificationSettings, KitNotificationType, Toast,
    ToastAction, ToastClose, ToastContent, ToastDescription, ToastIcon, ToastManager,
    ToastPlacement, ToastPortal, ToastProvider, ToastRecord, ToastTitle, ToastType, ToastViewport,
    Toaster, create_toast_manager,
};
pub use toggle::{KitToggle, Toggle, ToggleSize, ToggleVariant};
pub use toggle_group::{ToggleGroup, ToggleGroupItem, ToggleGroupOrientation};
pub use tooltip::{
    KitTooltip, Tooltip, TooltipAlign, TooltipContent, TooltipDefaults, TooltipProvider,
    TooltipSide, TooltipTrigger,
};
pub use trace::{
    Trace, TraceCollapsible, TraceCollapsibleContent, TraceCollapsibleTrigger, TraceDescription,
    TraceIcon, TraceLine, TraceStatus, TraceTitle,
};
pub use utils::{
    PresetRadius, PresetSpacing, PresetTokens, StyleComposeExt, card_surface_shadow,
    focus_ring_shadow, invalid_ring_shadow, overlay_surface_shadow, to_preset_uppercase,
};
