use utoipa::{
    Modify, OpenApi,
    openapi::security::{ApiKey, ApiKeyValue, HttpAuthScheme, HttpBuilder, SecurityScheme},
};

use crate::error::LocalizedErrorResponse;
use crate::handlers::{
    auth::{self, AuthResponse, LoginPayload, RegisterPayload, UpdateLocalePayload},
    families::{
        self, CreateFamilyRequest, CreateFamilyResponse, FamilyMemberView, FamilyWithRole,
        InviteMemberRequest, JoinFamilyRequest, JoinFamilyResponse, UpdateMemberRoleRequest,
        UpdateMemberRoleResponse,
    },
    projects::{
        self, AllocatePlanRequest, AllocatePlanResponse, CreateLoanRequest, CreatePlanRequest,
        CreateProjectRequest, ListLoansQuery, ListPlansQuery, ListProjectsQuery, RepayLoanRequest,
        RepayLoanResponse,
    },
    releases::{
        self, CheckReleaseQuery, CheckReleaseResponse, OtaBundleResponse, PublishOtaPayload,
        PublishOtaResponse, PublishReleasePayload, PublishReleaseResponse,
    },
    reports::{
        self, CreateShareLinkPayload, ReportSummaryQuery, ShareLinkResponse, SummaryReportResponse,
    },
    transactions::{
        self, CreateTransactionPayload, ListTransactionsQuery, TransactionExecutionResult,
    },
    wallets::{self, CreateWalletRequest, CustodyWalletView, ListWalletsQuery},
};
use umuryango_core_domain::models::{
    Family, FamilyMember, FamilyRole, FuturePlan, Loan, LoanDirection, LoanStatus,
    PendingFamilyInvite, Project, ProjectStatus, Transaction, TransactionType, User, Wallet,
    WalletChannel,
};
use umuryango_core_domain::reporting::{CustodianWalletSummary, MothersReport};
use umuryango_core_domain::scope::LedgerScope;
use umuryango_i18n::Locale;

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .description(Some("Enter your JWT token: Bearer <token>"))
                        .build(),
                ),
            );
            components.add_security_scheme(
                "admin_key",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("X-Admin-Key"))),
            );
        }
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(
        auth::register,
        auth::login,
        auth::update_locale,
        families::list_families,
        families::list_family_members,
        families::invite_member_by_phone,
        families::list_pending_invites,
        families::create_family,
        families::join_family,
        families::update_member_role,
        wallets::list_wallets,
        wallets::create_wallet,
        transactions::create_transaction,
        transactions::list_transactions,
        projects::list_projects,
        projects::create_project,
        projects::list_loans,
        projects::create_loan,
        projects::repay_loan,
        projects::list_plans,
        projects::create_plan,
        projects::allocate_plan,
        reports::get_summary_report,
        reports::create_share_link,
        reports::get_public_report,
        releases::check_release,
        releases::get_ota_bundle,
        releases::publish_release,
        releases::publish_ota_bundle,
    ),
    components(
        schemas(
            Locale,
            LocalizedErrorResponse,
            LedgerScope,
            FamilyRole,
            WalletChannel,
            TransactionType,
            ProjectStatus,
            LoanDirection,
            LoanStatus,
            User,
            Family,
            FamilyMember,
            FamilyMemberView,
            PendingFamilyInvite,
            Wallet,
            Transaction,
            Project,
            Loan,
            FuturePlan,
            CustodianWalletSummary,
            MothersReport,
            RegisterPayload,
            LoginPayload,
            UpdateLocalePayload,
            AuthResponse,
            CreateFamilyRequest,
            CreateFamilyResponse,
            InviteMemberRequest,
            JoinFamilyRequest,
            JoinFamilyResponse,
            UpdateMemberRoleRequest,
            UpdateMemberRoleResponse,
            FamilyWithRole,
            ListWalletsQuery,
            CreateWalletRequest,
            CustodyWalletView,
            CreateTransactionPayload,
            ListTransactionsQuery,
            TransactionExecutionResult,
            ListProjectsQuery,
            CreateProjectRequest,
            ListLoansQuery,
            CreateLoanRequest,
            RepayLoanRequest,
            RepayLoanResponse,
            ListPlansQuery,
            CreatePlanRequest,
            AllocatePlanRequest,
            AllocatePlanResponse,
            ReportSummaryQuery,
            CreateShareLinkPayload,
            SummaryReportResponse,
            ShareLinkResponse,
            CheckReleaseQuery,
            CheckReleaseResponse,
            PublishReleasePayload,
            PublishReleaseResponse,
            PublishOtaPayload,
            PublishOtaResponse,
            OtaBundleResponse,
        )
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "Auth", description = "Authentication, registration, and user locale endpoints"),
        (name = "Families", description = "Family group management, invitations, and role assignments"),
        (name = "Wallets", description = "Personal and family multi-channel custody wallets"),
        (name = "Transactions", description = "Idempotent double-entry transactions and handovers"),
        (name = "Projects", description = "Family development projects with milestone progress tracking"),
        (name = "Loans", description = "Zero-embarrassment personal and family lending ledger"),
        (name = "Plans", description = "Future financial plans and goals"),
        (name = "Reports", description = "Mother's Reports and magic links with WhatsApp summaries"),
        (name = "Releases", description = "Over-the-Air updates, binary releases, and runtime bundles")
    ),
    info(
        title = "Umuryango Ledger API",
        version = "1.0.0",
        description = "Bilingual (Kinyarwanda & English) Double-Entry Custody Accounting Ledger for Rwandan Families and Diaspora."
    )
)]
pub struct ApiDoc;
