//! Top workspace navigation bar and global command palette for all protected screens.
//!
//! Renders:
//! - Left cluster: collapsible sidebar toggle (`AppIcon::PanelLeft`), vertical separator,
//!   and active workspace title (`Personal workspace` in personal scope, or the active
//!   family name in family scope).
//! - Right cluster: interactive search pill (`Ctrl+K` / `⌘K`) opening [`CommandDialog`],
//!   `Families` workspace switcher (`DropdownMenu`), and notifications bell button.

use gpui_kit::component::input::{InputEvent, InputState};
use gpui_kit::{
    AppContext, Context, ElementId, Entity, Focusable as _, FontWeight, InteractiveElement,
    IntoElement, ParentElement, StatefulInteractiveElement, Styled, Subscription, Window, div, px,
    svg,
};
use umuryango_i18n::{Locale, t};

use crate::RootView;
use crate::common::{
    Button, ButtonSize, ButtonVariant, Command, CommandDialog, CommandEmpty, CommandGroup,
    CommandInput, CommandItem, CommandList, CommandSeparator, DropdownMenu, DropdownMenuContent,
    DropdownMenuGroup, DropdownMenuItem, DropdownMenuLabel, DropdownMenuSeparator,
    DropdownMenuTrigger, Kbd, PopoverAlign, PopoverSide, Separator, SidebarTrigger,
};
use crate::components::icon::{AppIcon, IconSize, IconTone, app_icon};
use crate::components::wallet_card::WalletCard;
use crate::state::{ActiveModal, AppState, Screen};
use crate::theme::{SemanticTheme, ThemeMode};

/// Persistent input state for the navbar's [`CommandDialog`] search box.
pub struct NavbarInputs {
    pub command_search: Entity<InputState>,
    _subscription: Subscription,
}

impl NavbarInputs {
    pub fn new(locale: Locale, window: &mut Window, cx: &mut Context<RootView>) -> Self {
        let placeholder = t(locale, "nav.search_placeholder");
        let command_search = cx.new(|cx| InputState::new(window, cx).placeholder(placeholder));

        let subscription =
            cx.subscribe(
                &command_search,
                |this, input, event: &InputEvent, cx| match event {
                    InputEvent::Change => {
                        this.state.command_query = input.read(cx).value().to_string();
                        cx.notify();
                    }
                    InputEvent::Focus | InputEvent::Blur | InputEvent::PressEnter { .. } => {
                        cx.notify();
                    }
                },
            );

        Self {
            command_search,
            _subscription: subscription,
        }
    }

    pub fn reset_and_focus(&self, locale: Locale, window: &mut Window, cx: &mut Context<RootView>) {
        let placeholder = t(locale, "nav.search_placeholder");
        self.command_search.update(cx, |input, cx| {
            input.set_placeholder(placeholder, window, cx);
            input.set_value("", window, cx);
            input.focus_handle(cx).focus(window, cx);
        });
    }
}

pub struct Navbar;

impl Navbar {
    /// Resolves the active workspace title shown in the navbar header:
    /// - Personal scope -> localized `"Personal workspace"`
    /// - Family scope -> active family name (falling back to localized `"Family Ledger"`).
    pub fn workspace_title(state: &AppState) -> String {
        let loc = state.locale;
        if state.active_scope.is_family() {
            state
                .active_family_name
                .as_deref()
                .map(str::trim)
                .filter(|name| !name.is_empty())
                .map(ToOwned::to_owned)
                .unwrap_or_else(|| t(loc, "scopes.family_ledger"))
        } else {
            t(loc, "scopes.personal_workspace")
        }
    }

    pub fn render(
        id: impl Into<ElementId>,
        state: &AppState,
        theme: &SemanticTheme,
        cx: &Context<RootView>,
    ) -> impl IntoElement {
        let loc = state.locale;
        let title = Self::workspace_title(state);
        let unread_count = state.unread_notifications_count;

        div()
            .id(id)
            .w_full()
            .h(px(52.0))
            .flex_shrink_0()
            .px(px(16.0))
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .gap(px(16.0))
            .bg(theme.bg_canvas)
            .border_b_1()
            .border_color(theme.border_subtle)
            .font_family(theme.font_family)
            // Left cluster: sidebar toggle + vertical divider + active workspace title
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(12.0))
                    .min_w_0()
                    .flex_1()
                    .child(
                        SidebarTrigger::new("navbar-sidebar-trigger")
                            .theme(theme)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.toggle_sidebar(cx);
                            })),
                    )
                    .child(Separator::vertical().theme(theme).h(px(16.0)))
                    .child(
                        div()
                            .min_w_0()
                            .text_base()
                            .font_weight(FontWeight::BOLD)
                            .text_color(theme.text_primary)
                            .truncate()
                            .child(title),
                    ),
            )
            // Right cluster: search trigger + Families workspace switcher + notifications bell
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(12.0))
                    .flex_shrink_0()
                    .child(render_search_trigger(loc, theme, cx))
                    .child(render_families_dropdown(state, theme, cx))
                    .child(render_notifications_bell(unread_count, theme, cx)),
            )
    }

    pub fn render_command_dialog(
        id: impl Into<ElementId>,
        state: &AppState,
        inputs: &NavbarInputs,
        theme: &SemanticTheme,
        cx: &Context<RootView>,
    ) -> impl IntoElement {
        let loc = state.locale;
        let query = state.command_query.trim().to_lowercase();
        let is_family_scope = state.active_scope.is_family();
        let active_family_id = state.active_scope.family_id();
        let is_dark = state.theme_mode == ThemeMode::Dark;

        let matches_query = |label: &str| query.is_empty() || label.to_lowercase().contains(&query);

        // 1. Workspaces group
        let mut workspace_items = Vec::new();
        let personal_label = t(loc, "scopes.personal_workspace");
        if matches_query(&personal_label) {
            workspace_items.push(
                CommandItem::new("cmd-ws-personal", personal_label)
                    .icon(AppIcon::PersonalWallet)
                    .checked(!is_family_scope)
                    .theme(theme)
                    .on_select(cx.listener(|this, _, _, cx| {
                        this.switch_to_personal_scope(cx);
                    }))
                    .into_any_element(),
            );
        }
        for fam in &state.families {
            if matches_query(&fam.name) {
                let fid = fam.id;
                workspace_items.push(
                    CommandItem::new(format!("cmd-ws-fam-{fid}"), fam.name.clone())
                        .icon(AppIcon::FamilyLedger)
                        .checked(active_family_id == Some(fid))
                        .theme(theme)
                        .on_select(cx.listener(move |this, _, _, cx| {
                            this.switch_to_family_by_id(fid, cx);
                        }))
                        .into_any_element(),
                );
            }
        }
        let setup_family_label = t(loc, "members.setup_family_btn");
        if matches_query(&setup_family_label) {
            workspace_items.push(
                CommandItem::new("cmd-ws-onboarding", setup_family_label)
                    .icon(AppIcon::Add)
                    .theme(theme)
                    .on_select(cx.listener(|this, _, _, cx| {
                        this.select_screen(Screen::Onboarding, cx);
                    }))
                    .into_any_element(),
            );
        }

        // 2. Navigation group
        let nav_entries = [
            (
                "cmd-nav-dashboard",
                Screen::Dashboard,
                AppIcon::Dashboard,
                t(loc, "nav.dashboard"),
            ),
            (
                "cmd-nav-transactions",
                Screen::Transactions,
                AppIcon::Transactions,
                t(loc, "nav.transactions"),
            ),
            (
                "cmd-nav-loans",
                Screen::Loans,
                AppIcon::Loans,
                t(loc, "nav.loans"),
            ),
            (
                "cmd-nav-plans",
                Screen::Plans,
                AppIcon::FuturePlans,
                t(loc, "nav.plans"),
            ),
            (
                "cmd-nav-projects",
                Screen::Projects,
                AppIcon::Projects,
                t(loc, "nav.projects"),
            ),
            (
                "cmd-nav-reports",
                Screen::Reports,
                AppIcon::Reports,
                t(loc, "nav.reports"),
            ),
            (
                "cmd-nav-members",
                Screen::Members,
                AppIcon::Members,
                t(loc, "nav.members"),
            ),
            (
                "cmd-nav-settings",
                Screen::Settings,
                AppIcon::Settings,
                t(loc, "nav.settings"),
            ),
        ];

        let mut nav_items = Vec::new();
        for (item_id, screen, icon, label) in nav_entries {
            if matches_query(&label) {
                nav_items.push(
                    CommandItem::new(item_id, label)
                        .icon(icon)
                        .checked(state.current_screen == screen)
                        .theme(theme)
                        .on_select(cx.listener(move |this, _, _, cx| {
                            this.select_screen(screen, cx);
                        }))
                        .into_any_element(),
                );
            }
        }

        // 3. Wallets in active workspace
        let mut wallet_items = Vec::new();
        for wallet in &state.wallets {
            let label = format!("{} ({})", wallet.name, WalletCard::format_balance(wallet));
            if matches_query(&label) {
                let icon = WalletCard::channel_icon(wallet.channel);
                wallet_items.push(
                    CommandItem::new(format!("cmd-wallet-{}", wallet.id), label)
                        .icon(icon)
                        .theme(theme)
                        .on_select(cx.listener(|this, _, _, cx| {
                            this.select_screen(Screen::Dashboard, cx);
                        }))
                        .into_any_element(),
                );
            }
        }

        // 4. Quick actions group
        let mut action_items = Vec::new();
        let quick_log_label = t(loc, "dashboard.quick_log_btn");
        if matches_query(&quick_log_label) {
            action_items.push(
                CommandItem::new("cmd-act-quick-entry", quick_log_label)
                    .icon(AppIcon::Add)
                    .theme(theme)
                    .on_select(cx.listener(|this, _, window, cx| {
                        this.state.show_command_palette = false;
                        this.open_quick_entry(window, cx);
                    }))
                    .into_any_element(),
            );
        }
        let add_wallet_label = t(loc, "wallets.add_wallet");
        if matches_query(&add_wallet_label) {
            action_items.push(
                CommandItem::new("cmd-act-add-wallet", add_wallet_label)
                    .icon(AppIcon::PersonalWallet)
                    .theme(theme)
                    .on_select(cx.listener(|this, _, window, cx| {
                        this.state.show_command_palette = false;
                        this.open_modal(ActiveModal::AddWallet, window, cx);
                    }))
                    .into_any_element(),
            );
        }
        let notifications_label = t(loc, "nav.notifications");
        if matches_query(&notifications_label) {
            action_items.push(
                CommandItem::new("cmd-act-notifications", notifications_label)
                    .icon(AppIcon::Bell)
                    .theme(theme)
                    .on_select(cx.listener(|this, _, _, cx| {
                        this.state.show_command_palette = false;
                        this.toggle_notifications(cx);
                    }))
                    .into_any_element(),
            );
        }
        let theme_label = t(
            loc,
            if is_dark {
                "common.light_mode"
            } else {
                "common.dark_mode"
            },
        );
        if matches_query(&theme_label) {
            action_items.push(
                CommandItem::new("cmd-act-theme", theme_label)
                    .icon(if is_dark { AppIcon::Sun } else { AppIcon::Moon })
                    .theme(theme)
                    .on_select(cx.listener(|this, _, _, cx| {
                        this.state.show_command_palette = false;
                        this.toggle_theme(cx);
                    }))
                    .into_any_element(),
            );
        }

        let has_any = !workspace_items.is_empty()
            || !nav_items.is_empty()
            || !wallet_items.is_empty()
            || !action_items.is_empty();

        let mut list = CommandList::new("navbar-command-list");
        if !has_any {
            list = list.child(CommandEmpty::new(t(loc, "nav.command_empty")).theme(theme));
        } else {
            let mut needs_sep = false;
            if !workspace_items.is_empty() {
                list = list.child(
                    CommandGroup::new()
                        .heading(t(loc, "nav.workspaces_group"))
                        .theme(theme)
                        .children(workspace_items),
                );
                needs_sep = true;
            }
            if !nav_items.is_empty() {
                if needs_sep {
                    list = list.child(CommandSeparator::new().theme(theme));
                }
                list = list.child(
                    CommandGroup::new()
                        .heading(t(loc, "nav.command_navigation"))
                        .theme(theme)
                        .children(nav_items),
                );
                needs_sep = true;
            }
            if !wallet_items.is_empty() {
                if needs_sep {
                    list = list.child(CommandSeparator::new().theme(theme));
                }
                list = list.child(
                    CommandGroup::new()
                        .heading(t(loc, "wallets.title"))
                        .theme(theme)
                        .children(wallet_items),
                );
                needs_sep = true;
            }
            if !action_items.is_empty() {
                if needs_sep {
                    list = list.child(CommandSeparator::new().theme(theme));
                }
                list = list.child(
                    CommandGroup::new()
                        .heading(t(loc, "nav.command_actions"))
                        .theme(theme)
                        .children(action_items),
                );
            }
        }

        CommandDialog::new(id)
            .open(state.show_command_palette)
            .theme(theme)
            .on_open_change(cx.listener(|this, next, window, cx| {
                this.set_command_palette_open(*next, window, cx);
            }))
            .child(
                Command::new("navbar-command-surface")
                    .theme(theme)
                    .child(CommandInput::new(&inputs.command_search).theme(theme))
                    .child(list),
            )
    }
}

fn search_shortcut_label() -> &'static str {
    if cfg!(target_os = "macos") {
        "⌘K"
    } else {
        "Ctrl K"
    }
}

fn render_search_trigger(
    loc: Locale,
    theme: &SemanticTheme,
    cx: &Context<RootView>,
) -> impl IntoElement {
    div()
        .id("navbar-search-trigger")
        .h(px(34.0))
        .w(px(260.0))
        .px(px(12.0))
        .flex()
        .flex_row()
        .items_center()
        .gap(px(8.0))
        .rounded(px(999.0))
        .border_1()
        .border_color(theme.border_subtle)
        .bg(theme.bg_canvas)
        .cursor_pointer()
        .hover(|s| s.bg(theme.bg_surface))
        .on_click(cx.listener(|this, _, window, cx| {
            this.open_command_palette(window, cx);
        }))
        .child(app_icon(
            AppIcon::Search,
            IconSize::Sm,
            IconTone::Muted,
            theme,
        ))
        .child(
            div()
                .flex_1()
                .min_w_0()
                .text_xs()
                .text_color(theme.text_muted)
                .truncate()
                .child(t(loc, "nav.search_placeholder")),
        )
        .child(
            Kbd::new(search_shortcut_label())
                .theme(theme)
                .h(px(20.0))
                .px(px(6.0))
                .rounded(px(4.0)),
        )
}

fn render_families_dropdown(
    state: &AppState,
    theme: &SemanticTheme,
    cx: &Context<RootView>,
) -> impl IntoElement {
    let loc = state.locale;
    let is_family_scope = state.active_scope.is_family();
    let active_family_id = state.active_scope.family_id();

    let mut personal_item =
        DropdownMenuItem::new("navbar-ws-personal", t(loc, "scopes.personal_workspace"))
            .icon(AppIcon::PersonalWallet)
            .uppercase(false)
            .theme(theme)
            .on_click(cx.listener(|this, _, _, cx| {
                this.switch_to_personal_scope(cx);
            }));
    if !is_family_scope {
        personal_item = personal_item.child(div().ml_auto().child(app_icon(
            AppIcon::Check,
            IconSize::Xs,
            IconTone::AccentPrimary,
            theme,
        )));
    }

    let mut group = DropdownMenuGroup::new()
        .child(DropdownMenuLabel::new(t(loc, "nav.workspaces_group")).theme(theme))
        .child(personal_item);

    for fam in &state.families {
        let fid = fam.id;
        let is_selected = active_family_id == Some(fid);
        let mut item = DropdownMenuItem::new(format!("navbar-ws-family-{fid}"), fam.name.clone())
            .icon(AppIcon::FamilyLedger)
            .uppercase(false)
            .theme(theme)
            .on_click(cx.listener(move |this, _, _, cx| {
                this.switch_to_family_by_id(fid, cx);
            }));
        if is_selected {
            item = item.child(div().ml_auto().child(app_icon(
                AppIcon::Check,
                IconSize::Xs,
                IconTone::AccentPrimary,
                theme,
            )));
        }
        group = group.child(item);
    }

    let trigger = DropdownMenuTrigger::new("navbar-families-trigger").child(
        div()
            .h(px(34.0))
            .px(px(10.0))
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.0))
            .hover(|s| s.bg(theme.bg_surface))
            .child(app_icon(
                AppIcon::Members,
                IconSize::Md,
                IconTone::Default,
                theme,
            ))
            .child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(theme.text_primary)
                    .child(t(loc, "nav.families_switcher")),
            )
            .child(chevrons_up_down(theme)),
    );

    let content = DropdownMenuContent::new()
        .side(PopoverSide::Bottom)
        .align(PopoverAlign::End)
        .side_offset(6.0)
        .w(px(240.0))
        .theme(theme)
        .child(group)
        .child(DropdownMenuSeparator::new().theme(theme))
        .child(
            DropdownMenuGroup::new().child(
                DropdownMenuItem::new("navbar-ws-create-join", t(loc, "members.setup_family_btn"))
                    .icon(AppIcon::Add)
                    .uppercase(false)
                    .theme(theme)
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.select_screen(Screen::Onboarding, cx);
                    })),
            ),
        );

    DropdownMenu::new("navbar-families-dropdown")
        .open(state.families_menu_open)
        .on_open_change(cx.listener(|this, next, _, cx| {
            this.set_families_menu_open(*next, cx);
        }))
        .trigger(trigger)
        .content(content)
}

fn chevrons_up_down(theme: &SemanticTheme) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .gap(px(0.0))
        .child(
            svg()
                .path(AppIcon::ChevronUp.asset_path())
                .size(px(8.0))
                .text_color(theme.text_muted),
        )
        .child(
            svg()
                .path(AppIcon::ChevronDown.asset_path())
                .size(px(8.0))
                .text_color(theme.text_muted),
        )
}

fn render_notifications_bell(
    unread_count: usize,
    theme: &SemanticTheme,
    cx: &Context<RootView>,
) -> impl IntoElement {
    div()
        .relative()
        .flex()
        .items_center()
        .justify_center()
        .child(
            Button::new("navbar-notifications-btn")
                .variant(ButtonVariant::Ghost)
                .size(ButtonSize::IconSm)
                .theme(theme)
                .on_click(cx.listener(|this, _, _, cx| {
                    this.toggle_notifications(cx);
                }))
                .child(app_icon(
                    AppIcon::Bell,
                    IconSize::Md,
                    IconTone::Muted,
                    theme,
                )),
        )
        .children((unread_count > 0).then(|| {
            div()
                .absolute()
                .top(px(5.0))
                .right(px(5.0))
                .size(px(7.0))
                .rounded(px(999.0))
                .bg(theme.accent_primary)
        }))
}

#[cfg(test)]
mod tests {
    use umuryango_core_domain::scope::LedgerScope;
    use umuryango_i18n::Locale;
    use uuid::Uuid;

    use super::Navbar;
    use crate::state::AppState;

    #[test]
    fn workspace_title_shows_personal_workspace_when_personal_scope_is_active() {
        let uid = Uuid::new_v4();
        let mut state = AppState {
            locale: Locale::En,
            user_id: uid,
            active_scope: LedgerScope::Personal { user_id: uid },
            active_family_name: Some("Ignored Family".to_string()),
            ..Default::default()
        };
        assert_eq!(Navbar::workspace_title(&state), "Personal workspace");

        state.locale = Locale::Rw;
        assert_eq!(Navbar::workspace_title(&state), "Konti yanjye bwite");
    }

    #[test]
    fn workspace_title_shows_active_family_name_when_family_scope_is_active() {
        let fid = Uuid::new_v4();
        let mut state = AppState {
            locale: Locale::En,
            active_scope: LedgerScope::Family { family_id: fid },
            active_family_id: Some(fid),
            active_family_name: Some("Abahire Family".to_string()),
            ..Default::default()
        };
        assert_eq!(Navbar::workspace_title(&state), "Abahire Family");

        state.active_family_name = None;
        assert_eq!(Navbar::workspace_title(&state), "Family Ledger");

        state.locale = Locale::Rw;
        assert_eq!(Navbar::workspace_title(&state), "Konti y'Umuryango");
    }
}
