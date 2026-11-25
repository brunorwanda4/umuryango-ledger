//! Local persistence for active screen, auth mode (Login vs Register), onboarding step, and pre-auth UI preferences.

use crate::db_facade::LocalDatabase;
#[cfg(not(target_arch = "wasm32"))]
use crate::state::SESSION_KEY_LOCALE;
use crate::state::{AppState, OnboardingStep, Screen};
#[cfg(not(target_arch = "wasm32"))]
use crate::theme::{SemanticTheme, ThemeMode};
#[cfg(not(target_arch = "wasm32"))]
use umuryango_i18n::Locale;

#[cfg(not(target_arch = "wasm32"))]
const NAV_KEY_CURRENT_SCREEN: &str = "nav.current_screen";
#[cfg(not(target_arch = "wasm32"))]
const NAV_KEY_AUTH_MODE_REGISTER: &str = "nav.auth_mode_register";
#[cfg(not(target_arch = "wasm32"))]
const NAV_KEY_ONBOARDING_STEP: &str = "nav.onboarding_step";
#[cfg(not(target_arch = "wasm32"))]
const UI_KEY_THEME_MODE: &str = "ui.theme_mode";

impl OnboardingStep {
    /// Stable lowercase slug persisted in local SQLite `app_config`.
    pub fn as_slug(self) -> &'static str {
        match self {
            OnboardingStep::Choose => "choose",
            OnboardingStep::CreateFamily => "create",
            OnboardingStep::JoinFamily => "join",
        }
    }

    /// Parses a persisted slug back into an `OnboardingStep`.
    pub fn from_slug(slug: &str) -> Option<Self> {
        match slug.trim().to_ascii_lowercase().as_str() {
            "choose" => Some(OnboardingStep::Choose),
            "create" => Some(OnboardingStep::CreateFamily),
            "join" => Some(OnboardingStep::JoinFamily),
            _ => None,
        }
    }
}

impl Screen {
    /// Stable lowercase slug persisted in local SQLite `app_config`.
    pub fn as_slug(self) -> &'static str {
        match self {
            Screen::Auth => "auth",
            Screen::Onboarding => "onboarding",
            Screen::Dashboard => "dashboard",
            Screen::Transactions => "transactions",
            Screen::Reports => "reports",
            Screen::Projects => "projects",
            Screen::Loans => "loans",
            Screen::Plans => "plans",
            Screen::Members => "members",
            Screen::Settings => "settings",
        }
    }

    /// Parses a persisted slug back into a `Screen`.
    pub fn from_slug(slug: &str) -> Option<Self> {
        match slug.trim().to_ascii_lowercase().as_str() {
            "auth" => Some(Screen::Auth),
            "onboarding" => Some(Screen::Onboarding),
            "dashboard" => Some(Screen::Dashboard),
            "transactions" => Some(Screen::Transactions),
            "reports" => Some(Screen::Reports),
            "projects" => Some(Screen::Projects),
            "loans" => Some(Screen::Loans),
            "plans" => Some(Screen::Plans),
            "members" => Some(Screen::Members),
            "settings" => Some(Screen::Settings),
            _ => None,
        }
    }

    /// Whether this screen requires an authenticated session.
    pub fn requires_auth(self) -> bool {
        !matches!(self, Screen::Auth)
    }
}

impl AppState {
    /// Persists the current screen, auth mode (Login vs Register), onboarding step, and locale into SQLite `app_config`.
    #[cfg(target_arch = "wasm32")]
    pub fn save_navigation_to_db(&self, _db: &LocalDatabase) {}

    #[cfg(not(target_arch = "wasm32"))]
    pub fn save_navigation_to_db(&self, db: &LocalDatabase) {
        let _ = db.set_config(NAV_KEY_CURRENT_SCREEN, self.current_screen.as_slug());
        let _ = db.set_config(
            NAV_KEY_AUTH_MODE_REGISTER,
            if self.auth_mode_register {
                "true"
            } else {
                "false"
            },
        );
        let _ = db.set_config(NAV_KEY_ONBOARDING_STEP, self.onboarding_step.as_slug());
        let _ = db.set_config(SESSION_KEY_LOCALE, self.locale.as_str());
    }

    /// Persists the global theme mode fallback (so Light/Dark also persists before login).
    #[cfg(target_arch = "wasm32")]
    pub fn save_theme_mode_to_db(&self, _db: &LocalDatabase) {}

    #[cfg(not(target_arch = "wasm32"))]
    pub fn save_theme_mode_to_db(&self, db: &LocalDatabase) {
        let mode_str = match self.theme_mode {
            ThemeMode::Dark => "dark",
            ThemeMode::Light => "light",
        };
        let _ = db.set_config(UI_KEY_THEME_MODE, mode_str);
    }

    /// Restores the last visited screen, Login/Register mode, onboarding step, and pre-auth UI preferences.
    ///
    /// Must be called after `restore_session_from_db`:
    /// - When unauthenticated, never restores an authenticated-only screen (stays on `Screen::Auth`),
    ///   while honoring `auth_mode_register`, locale, and theme mode.
    /// - When authenticated, restores the last authenticated screen (defaulting to `Screen::Dashboard`).
    #[cfg(target_arch = "wasm32")]
    pub fn restore_navigation_from_db(&mut self, _db: &LocalDatabase) {}

    #[cfg(not(target_arch = "wasm32"))]
    pub fn restore_navigation_from_db(&mut self, db: &LocalDatabase) {
        if let Ok(Some(loc)) = db.get_config(SESSION_KEY_LOCALE) {
            self.locale = match loc.as_str() {
                "en" => Locale::En,
                _ => Locale::Rw,
            };
        }

        if let Ok(Some(register_flag)) = db.get_config(NAV_KEY_AUTH_MODE_REGISTER) {
            self.auth_mode_register = register_flag.eq_ignore_ascii_case("true");
        }

        if !self.is_authenticated {
            if let Ok(Some(mode)) = db.get_config(UI_KEY_THEME_MODE) {
                self.theme_mode = if mode.eq_ignore_ascii_case("dark") {
                    ThemeMode::Dark
                } else {
                    ThemeMode::Light
                };
                self.theme = SemanticTheme::from_config(&self.theme_config, self.theme_mode);
                crate::safe_area::platform::set_system_bars_contrast(
                    self.theme_mode != ThemeMode::Dark,
                );
            }
            self.onboarding_step = OnboardingStep::Choose;
            self.current_screen = Screen::Auth;
            return;
        }

        let restored_screen = db
            .get_config(NAV_KEY_CURRENT_SCREEN)
            .ok()
            .flatten()
            .and_then(|slug| Screen::from_slug(&slug))
            .filter(|screen| screen.requires_auth())
            .unwrap_or(Screen::Dashboard);

        self.onboarding_step = if restored_screen == Screen::Onboarding {
            db.get_config(NAV_KEY_ONBOARDING_STEP)
                .ok()
                .flatten()
                .and_then(|slug| OnboardingStep::from_slug(&slug))
                .unwrap_or(OnboardingStep::Choose)
        } else {
            OnboardingStep::Choose
        };

        self.current_screen = restored_screen;
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use uuid::Uuid;

    use super::*;
    use crate::db_facade::LocalDatabase;

    #[test]
    fn screen_slug_roundtrip_covers_all_variants() {
        let screens = [
            Screen::Auth,
            Screen::Onboarding,
            Screen::Dashboard,
            Screen::Transactions,
            Screen::Reports,
            Screen::Projects,
            Screen::Loans,
            Screen::Plans,
            Screen::Members,
            Screen::Settings,
        ];
        for screen in screens {
            assert_eq!(Screen::from_slug(screen.as_slug()), Some(screen));
        }
        assert_eq!(Screen::from_slug("unknown-screen"), None);

        for step in [
            OnboardingStep::Choose,
            OnboardingStep::CreateFamily,
            OnboardingStep::JoinFamily,
        ] {
            assert_eq!(OnboardingStep::from_slug(step.as_slug()), Some(step));
        }
        assert_eq!(OnboardingStep::from_slug("unknown-step"), None);
    }

    #[test]
    fn restores_register_mode_when_closed_on_register_screen() {
        let db = LocalDatabase::open_in_memory().expect("in-memory db");

        let mut first_run = AppState::default();
        first_run.current_screen = Screen::Auth;
        first_run.auth_mode_register = true;
        first_run.locale = Locale::En;
        first_run.set_theme_mode(ThemeMode::Dark, Some(&db));
        first_run.save_navigation_to_db(&db);

        let mut reopened = AppState::default();
        assert!(reopened.restore_session_from_db(&db).is_none());
        reopened.restore_navigation_from_db(&db);

        assert!(!reopened.is_authenticated);
        assert_eq!(reopened.current_screen, Screen::Auth);
        assert!(reopened.auth_mode_register);
        assert_eq!(reopened.locale, Locale::En);
        assert_eq!(reopened.theme_mode, ThemeMode::Dark);
    }

    #[test]
    fn unauthenticated_startup_never_restores_protected_screen() {
        let db = LocalDatabase::open_in_memory().expect("in-memory db");
        db.set_config(NAV_KEY_CURRENT_SCREEN, "transactions")
            .expect("write screen");
        db.set_config(NAV_KEY_AUTH_MODE_REGISTER, "true")
            .expect("write register flag");

        let mut reopened = AppState::default();
        assert!(reopened.restore_session_from_db(&db).is_none());
        reopened.restore_navigation_from_db(&db);

        assert_eq!(reopened.current_screen, Screen::Auth);
        assert!(reopened.auth_mode_register);
    }

    #[test]
    fn authenticated_startup_restores_last_visited_screen() {
        let db = LocalDatabase::open_in_memory().expect("in-memory db");
        let uid = Uuid::new_v4();

        let mut first_run = AppState::new(uid, "Alice".to_string());
        first_run.user_phone = "+250788000001".to_string();
        first_run.auth_token = Some("jwt-test-token".to_string());
        first_run.is_authenticated = true;
        first_run.current_screen = Screen::Projects;
        first_run.auth_mode_register = false;
        first_run.save_session_to_db(&db);
        first_run.save_navigation_to_db(&db);

        let mut reopened = AppState::default();
        let token = reopened.restore_session_from_db(&db);
        reopened.restore_navigation_from_db(&db);

        assert_eq!(token.as_deref(), Some("jwt-test-token"));
        assert!(reopened.is_authenticated);
        assert_eq!(reopened.current_screen, Screen::Projects);
        assert!(!reopened.auth_mode_register);
    }

    #[test]
    fn authenticated_startup_restores_onboarding_screen_and_step() {
        let db = LocalDatabase::open_in_memory().expect("in-memory db");
        let uid = Uuid::new_v4();

        let mut first_run = AppState::new(uid, "Bob".to_string());
        first_run.auth_token = Some("jwt-onboarding".to_string());
        first_run.is_authenticated = true;
        first_run.current_screen = Screen::Onboarding;
        first_run.onboarding_step = OnboardingStep::JoinFamily;
        first_run.save_session_to_db(&db);
        first_run.save_navigation_to_db(&db);

        let mut reopened = AppState::default();
        assert!(reopened.restore_session_from_db(&db).is_some());
        reopened.restore_navigation_from_db(&db);

        assert_eq!(reopened.current_screen, Screen::Onboarding);
        assert_eq!(reopened.onboarding_step, OnboardingStep::JoinFamily);
    }

    #[test]
    fn logout_resets_saved_navigation_back_to_login() {
        let db = LocalDatabase::open_in_memory().expect("in-memory db");
        let uid = Uuid::new_v4();

        let mut state = AppState::new(uid, "Alice".to_string());
        state.auth_token = Some("jwt-token".to_string());
        state.is_authenticated = true;
        state.current_screen = Screen::Loans;
        state.auth_mode_register = true;
        state.onboarding_step = OnboardingStep::CreateFamily;
        state.save_session_to_db(&db);
        state.save_navigation_to_db(&db);

        AppState::clear_session_in_db(&db);
        state.reset_session();
        state.save_navigation_to_db(&db);

        let mut reopened = AppState::default();
        assert!(reopened.restore_session_from_db(&db).is_none());
        reopened.restore_navigation_from_db(&db);

        assert_eq!(reopened.current_screen, Screen::Auth);
        assert!(!reopened.auth_mode_register);
        assert_eq!(reopened.onboarding_step, OnboardingStep::Choose);
    }
}
