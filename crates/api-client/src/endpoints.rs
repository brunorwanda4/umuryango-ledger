pub struct ApiEndpoints;

impl ApiEndpoints {
    pub const LOGIN: &'static str = "/api/v1/auth/login";
    pub const REGISTER: &'static str = "/api/v1/auth/register";
    pub const FAMILIES: &'static str = "/api/v1/families";
    pub const WALLETS: &'static str = "/api/v1/wallets";
    pub const CATEGORIES: &'static str = "/api/v1/categories";
    pub const TRANSACTIONS: &'static str = "/api/v1/transactions";
    pub const HANDOVERS: &'static str = "/api/v1/transactions/handovers";
    pub const PROJECTS: &'static str = "/api/v1/projects";
    pub const LOANS: &'static str = "/api/v1/loans";
    pub const PLANS: &'static str = "/api/v1/plans";
    pub const REPORTS_SUMMARY: &'static str = "/api/v1/reports/summary";
    pub const ZERO_LOGIN_REPORT: &'static str = "/api/v1/reports/zero-login";
}
