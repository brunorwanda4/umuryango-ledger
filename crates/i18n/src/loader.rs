use std::collections::HashMap;
use std::sync::{OnceLock, RwLock};

use crate::keys::Locale;

pub static EN_JSON: &str = include_str!("../../../locales/en.json");
pub static RW_JSON: &str = include_str!("../../../locales/rw.json");

static BUNDLE_STORE: OnceLock<RwLock<HashMap<Locale, serde_json::Value>>> = OnceLock::new();

fn get_store() -> &'static RwLock<HashMap<Locale, serde_json::Value>> {
    BUNDLE_STORE.get_or_init(|| {
        let mut map = HashMap::new();
        let en_val: serde_json::Value =
            serde_json::from_str(EN_JSON).expect("locales/en.json must be valid JSON");
        let rw_val: serde_json::Value =
            serde_json::from_str(RW_JSON).expect("locales/rw.json must be valid JSON");
        map.insert(Locale::En, en_val);
        map.insert(Locale::Rw, rw_val);
        RwLock::new(map)
    })
}

/// Allows hot-reloading updated OTA JSON translation bundles at runtime
pub fn reload_bundle(locale: Locale, json_str: &str) -> Result<(), serde_json::Error> {
    let parsed: serde_json::Value = serde_json::from_str(json_str)?;
    let store = get_store();
    if let Ok(mut write_guard) = store.write() {
        write_guard.insert(locale, parsed);
    }
    Ok(())
}

/// Traverses a JSON value tree using dot notation (e.g. "errors.insufficient_custody")
pub fn resolve_dot_path(root: &serde_json::Value, path: &str) -> Option<String> {
    let mut current = root;
    for segment in path.split('.') {
        match current {
            serde_json::Value::Object(map) => {
                current = map.get(segment)?;
            }
            _ => return None,
        }
    }

    match current {
        serde_json::Value::String(s) => Some(s.clone()),
        _ => None,
    }
}

/// Interpolates `{variable}` or `{$variable}` placeholders in template strings
pub fn interpolate(template: &str, args: &[(&str, &str)]) -> String {
    if args.is_empty() {
        return template.to_string();
    }

    let mut result = template.to_string();
    for (k, v) in args {
        let pat_standard = format!("{{{}}}", k);
        let pat_dollar = format!("{{${}}}", k);
        if result.contains(&pat_standard) {
            result = result.replace(&pat_standard, v);
        }
        if result.contains(&pat_dollar) {
            result = result.replace(&pat_dollar, v);
        }
    }
    result
}

/// Maps legacy hyphenated Fluent keys to canonical dot-paths for 100% backward compatibility
pub fn canonicalize_key(key: &str) -> &str {
    match key {
        "app-name" => "common.app_name",
        "nav-dashboard" => "dashboard.title",
        "nav-transactions" => "transactions.title",
        "nav-wallets" => "wallets.title",
        "nav-projects" => "projects.title",
        "nav-loans" => "loans.title",
        "nav-plans" => "future_plans.title",
        "nav-reports" => "report.title",
        "nav-members" => "members.title",
        "personal-scope" => "scopes.personal_scope",
        "family-scope" => "scopes.family_scope",
        "role-head" => "members.role_head",
        "role-custodian" => "members.role_custodian",
        "role-member" => "members.role_member",
        "income" => "transactions.income",
        "expense" => "transactions.expense",
        "handover" => "transactions.handover",
        "custodian" => "wallets.held_by",
        "opening-balance" => "report.opening_balance",
        "closing-balance" => "report.closing_balance",
        "held-by" => "wallets.held_by",
        "authorized-by" => "transactions.authorized_by",
        "paid-to" => "transactions.paid_to",
        "channel-bank" => "wallets.bank",
        "channel-momo" => "wallets.mobile_money",
        "channel-cash" => "wallets.cash_in_hand",
        "mothers-report-title" => "report.mothers_report_title",
        "total-income" => "report.total_income",
        "total-expenses" => "report.total_expenses",
        "cash-custody-breakdown" => "report.closing_balance_by_person",
        "active-projects" => "report.active_projects",
        "loans-summary" => "report.loans_summary",
        "future-plans" => "report.future_plans",
        "report-generated-at" => "report.generated_at",
        "report-period" => "report.period",
        "notif-income-added" => "notifications.income_added",
        "notif-expense-logged" => "notifications.expense_logged",
        "notif-handover-completed" => "notifications.handover_completed",
        "notif-project-loan-updated" => "notifications.project_loan_updated",
        "notif-app-update-available" => "notifications.app_update_available",
        "notif-app-update-mandatory" => "notifications.app_update_mandatory",
        "update-ready-banner" => "updater.update_ready",
        "update-downloading" => "updater.downloading",
        "update-mandatory-title" => "updater.mandatory_title",
        "update-restart-now" => "updater.restart_now",
        "login-title" => "auth.login_title",
        "register-title" => "auth.register_title",
        "full-name" => "auth.full_name",
        "phone-number" => "auth.phone_number",
        "gender" => "auth.gender",
        "gender-male" => "auth.gender_male",
        "gender-female" => "auth.gender_female",
        "gender-other" => "auth.gender_other",
        "password" => "auth.password",
        "confirm-password" => "auth.confirm_password",
        "create-account" => "auth.create_account_btn",
        "already-have-account" => "auth.switch_to_login",
        "onboarding-title" => "onboarding.title",
        "create-family" => "onboarding.create_family",
        "family-name" => "onboarding.family_name",
        "add-member-phone" => "onboarding.add_member_phone",
        "add-phone-btn" => "onboarding.add_another_phone",
        "join-family" => "onboarding.join_family",
        "pending-family-invites" => "onboarding.pending_invites_banner",
        "enter-4-char-code" => "onboarding.enter_4_char_code",
        "skip-for-now" => "onboarding.skip_for_now",
        "continue-to-dashboard" => "onboarding.continue_to_dashboard",
        "you-were-added-to" => "onboarding.you_were_added_to",
        "join-btn" => "onboarding.join_now",
        "error-insufficient-custody" => "errors.insufficient_custody",
        "error-unauthorized-role" => "errors.unauthorized_role",
        "error-invalid-handover" => "errors.invalid_handover",
        "error-scope-mismatch" => "errors.scope_mismatch",
        "error-invalid-amount" => "errors.invalid_amount",
        "error-not-found" => "errors.not_found",
        "error-validation" => "errors.validation",
        "error-password-mismatch" => "errors.password_mismatch",
        "error-phone-already-exists" => "errors.phone_already_exists",
        "error-invalid-credentials" => "errors.invalid_credentials",
        "error-invalid-invite-code" => "errors.invalid_invite_code",
        other => other,
    }
}

/// Resolves a localized template with fallback to Locale::En, then interpolates args
pub fn lookup_and_format(locale: Locale, key: &str, args: &[(&str, &str)]) -> String {
    let canonical_key = canonicalize_key(key);
    let store = get_store();
    let read_guard = match store.read() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };

    // 1. Try requested locale
    if let Some(root) = read_guard.get(&locale) {
        if let Some(template) = resolve_dot_path(root, canonical_key) {
            return interpolate(&template, args);
        }
    }

    // 2. Fallback to English if requested wasn't English
    if locale != Locale::En {
        if let Some(root) = read_guard.get(&Locale::En) {
            if let Some(template) = resolve_dot_path(root, canonical_key) {
                return interpolate(&template, args);
            }
        }
    }

    // 3. Fallback to key itself if not found
    key.to_string()
}
