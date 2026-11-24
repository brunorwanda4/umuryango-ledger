use chrono::{DateTime, Utc};
use reqwest::Client;
use reqwest::header::{ACCEPT_LANGUAGE, AUTHORIZATION, HeaderMap, HeaderValue};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;
use umuryango_core_domain::models::{
    Family, FamilyMemberView, FamilyRole, FamilyWithRole, FuturePlan, Loan, LoanDirection,
    PendingFamilyInvite, Project, Transaction, TransactionType, User, Wallet, WalletChannel,
};
use umuryango_core_domain::reporting::MothersReport;
use umuryango_core_domain::scope::LedgerScope;
use umuryango_i18n::Locale;
use uuid::Uuid;

use crate::endpoints::ApiEndpoints;

#[derive(Debug, Error)]
pub enum ApiClientError {
    #[error("HTTP transport error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Server error ({code}): {message}")]
    ServerError {
        code: String,
        message: String,
        status: u16,
    },

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("WebSocket error: {0}")]
    WebSocket(String),

    #[error("Client error: {0}")]
    Generic(String),
}

impl ApiClientError {
    /// Human-readable message suitable for displaying in UI toast notifications.
    pub fn user_message(&self, locale: Locale) -> String {
        match self {
            ApiClientError::ServerError { message, .. } if !message.trim().is_empty() => {
                message.clone()
            }
            ApiClientError::Reqwest(_) => umuryango_i18n::t(locale, "errors.network_offline"),
            other => other.to_string(),
        }
    }

    pub fn is_unauthorized(&self) -> bool {
        matches!(self, ApiClientError::ServerError { status: 401, .. })
    }

    pub fn is_network_error(&self) -> bool {
        matches!(self, ApiClientError::Reqwest(_))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    #[serde(default)]
    pub user: Option<User>,
    #[serde(default)]
    pub user_id: Option<Uuid>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub full_name: Option<String>,
    #[serde(default)]
    pub locale: Option<Locale>,
    #[serde(default)]
    pub families: Vec<FamilyWithRole>,
    #[serde(default)]
    pub pending_invites: Vec<PendingFamilyInvite>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub full_name: String,
    pub phone: Option<String>,
    pub phone_number: Option<String>,
    pub gender: Option<String>,
    pub password: String,
    pub confirm_password: Option<String>,
    pub email: Option<String>,
    pub preferred_locale: Option<Locale>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub phone: Option<String>,
    pub login: Option<String>,
    pub email: Option<String>,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFamilyRequest {
    pub name: String,
    pub description: Option<String>,
    pub currency: Option<String>,
    #[serde(default)]
    pub member_phones: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFamilyResponse {
    pub family: Family,
    pub invite_code: String,
    #[serde(default)]
    pub added_invites_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinFamilyRequest {
    pub invite_code: Option<String>,
    pub invite_id: Option<Uuid>,
    pub display_name: Option<String>,
    pub role: Option<FamilyRole>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinFamilyResponse {
    pub status: String,
    pub family_id: Uuid,
    pub family_name: String,
    pub role: FamilyRole,
    pub family: Family,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWalletRequest {
    pub name: String,
    pub channel: WalletChannel,
    pub currency: String,
    pub account_number: Option<String>,
    pub custodian_id: Option<Uuid>,
    pub scope: String,
    pub family_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustodyWalletView {
    pub id: Uuid,
    pub scope: LedgerScope,
    pub channel: WalletChannel,
    pub name: String,
    pub account_number: Option<String>,
    pub currency: String,
    pub balance: Decimal,
    pub custodian_id: Option<Uuid>,
    pub custodian_name: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl CustodyWalletView {
    pub fn into_wallet(self) -> (Wallet, Option<String>) {
        let custodian_name = self.custodian_name;
        let wallet = Wallet {
            id: self.id,
            scope: self.scope,
            channel: self.channel,
            name: self.name,
            account_number: self.account_number,
            currency: self.currency,
            balance: self.balance,
            custodian_id: self.custodian_id,
            is_active: self.is_active,
            created_at: self.created_at,
            updated_at: self.updated_at,
        };
        (wallet, custodian_name)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTransactionRequest {
    pub scope: String,
    pub family_id: Option<Uuid>,
    pub source_wallet_id: Uuid,
    pub destination_wallet_id: Option<Uuid>,
    pub tx_type: TransactionType,
    pub amount: Decimal,
    pub category: String,
    pub description: Option<String>,
    pub paid_to: Option<String>,
    pub receipt_url: Option<String>,
    pub tx_date: Option<DateTime<Utc>>,
    pub idempotency_key: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionExecutionResult {
    pub transaction: Transaction,
    pub source_wallet_balance: Decimal,
    pub destination_wallet_balance: Option<Decimal>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProjectRequest {
    pub family_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub target_amount: Decimal,
    pub start_date: Option<chrono::NaiveDate>,
    pub target_end_date: Option<chrono::NaiveDate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLoanRequest {
    pub scope: String,
    pub family_id: Option<Uuid>,
    pub counterparty_name: String,
    pub principal_amount: Decimal,
    pub interest_rate: Option<Decimal>,
    pub due_date: Option<chrono::NaiveDate>,
    pub direction: LoanDirection,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePlanRequest {
    pub scope: String,
    pub family_id: Option<Uuid>,
    pub title: String,
    pub description: Option<String>,
    pub estimated_cost: Decimal,
    pub target_date: Option<chrono::NaiveDate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareLinkResponse {
    pub share_token: String,
    pub share_url: String,
    #[serde(default)]
    pub whatsapp_preview_text: String,
    pub expires_at: DateTime<Utc>,
}

#[derive(Clone)]
pub struct ApiClient {
    base_url: String,
    pub ws_url: String,
    http: Client,
    auth_token: Arc<RwLock<Option<String>>>,
    locale: Arc<RwLock<Locale>>,
}

// Backward compatibility alias
pub type UmuryangoApiClient = ApiClient;

impl ApiClient {
    pub fn new(base_url: String, ws_url: String, locale: Locale) -> Self {
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            ws_url: ws_url.trim_end_matches('/').to_string(),
            http: Client::builder().build().unwrap_or_default(),
            auth_token: Arc::new(RwLock::new(None)),
            locale: Arc::new(RwLock::new(locale)),
        }
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub async fn set_token(&self, token: Option<String>) {
        let mut t = self.auth_token.write().await;
        *t = token;
    }

    pub async fn set_locale(&self, locale: Locale) {
        let mut l = self.locale.write().await;
        *l = locale;
    }

    pub async fn get_token(&self) -> Option<String> {
        self.auth_token.read().await.clone()
    }

    pub async fn get_locale(&self) -> Locale {
        *self.locale.read().await
    }

    async fn build_headers(&self, idempotency_key: Option<Uuid>) -> HeaderMap {
        let mut headers = HeaderMap::new();
        let loc = *self.locale.read().await;
        headers.insert(
            ACCEPT_LANGUAGE,
            HeaderValue::from_str(loc.as_str()).unwrap_or_else(|_| HeaderValue::from_static("rw")),
        );

        if let Some(token) = self.auth_token.read().await.as_ref() {
            if let Ok(val) = HeaderValue::from_str(&format!("Bearer {}", token)) {
                headers.insert(AUTHORIZATION, val);
            }
        }

        if let Some(key) = idempotency_key {
            if let Ok(val) = HeaderValue::from_str(&key.to_string()) {
                headers.insert("X-Idempotency-Key", val);
            }
        }

        headers
    }

    async fn handle_response<T: for<'de> Deserialize<'de>>(
        resp: reqwest::Response,
    ) -> Result<T, ApiClientError> {
        let status = resp.status();
        if status.is_success() {
            let parsed = resp.json::<T>().await?;
            Ok(parsed)
        } else {
            #[derive(Deserialize)]
            struct ErrorBody {
                code: Option<String>,
                message: Option<String>,
            }
            let u16_status = status.as_u16();
            let body_text = resp.text().await.unwrap_or_default();
            if let Ok(parsed_err) = serde_json::from_str::<ErrorBody>(&body_text) {
                Err(ApiClientError::ServerError {
                    code: parsed_err.code.unwrap_or_else(|| "ERROR".to_string()),
                    message: parsed_err.message.unwrap_or(body_text),
                    status: u16_status,
                })
            } else {
                Err(ApiClientError::ServerError {
                    code: format!("HTTP_{}", u16_status),
                    message: body_text,
                    status: u16_status,
                })
            }
        }
    }

    // ================= AUTH =================

    pub async fn register(&self, req: &RegisterRequest) -> Result<AuthResponse, ApiClientError> {
        let url = format!("{}{}", self.base_url, ApiEndpoints::REGISTER);
        let resp = self
            .http
            .post(&url)
            .headers(self.build_headers(None).await)
            .json(req)
            .send()
            .await?;
        let auth = Self::handle_response::<AuthResponse>(resp).await?;
        self.set_token(Some(auth.token.clone())).await;
        Ok(auth)
    }

    pub async fn login(&self, req: &LoginRequest) -> Result<AuthResponse, ApiClientError> {
        let url = format!("{}{}", self.base_url, ApiEndpoints::LOGIN);
        let resp = self
            .http
            .post(&url)
            .headers(self.build_headers(None).await)
            .json(req)
            .send()
            .await?;
        let auth = Self::handle_response::<AuthResponse>(resp).await?;
        self.set_token(Some(auth.token.clone())).await;
        Ok(auth)
    }

    pub async fn update_locale(&self, locale: Locale) -> Result<(), ApiClientError> {
        let url = format!("{}{}", self.base_url, "/api/v1/auth/locale");
        let resp = self
            .http
            .patch(&url)
            .headers(self.build_headers(None).await)
            .json(&serde_json::json!({ "locale": locale.as_str() }))
            .send()
            .await?;
        if resp.status().is_success() {
            self.set_locale(locale).await;
            Ok(())
        } else {
            Self::handle_response::<serde_json::Value>(resp).await?;
            Ok(())
        }
    }

    // ================= FAMILIES =================

    pub async fn list_families(&self) -> Result<Vec<FamilyWithRole>, ApiClientError> {
        let url = format!("{}{}", self.base_url, ApiEndpoints::FAMILIES);
        let resp = self
            .http
            .get(&url)
            .headers(self.build_headers(None).await)
            .send()
            .await?;
        Self::handle_response(resp).await
    }

    pub async fn list_family_members(
        &self,
        family_id: Uuid,
    ) -> Result<Vec<FamilyMemberView>, ApiClientError> {
        let url = format!(
            "{}{}/{}/members",
            self.base_url,
            ApiEndpoints::FAMILIES,
            family_id
        );
        let resp = self
            .http
            .get(&url)
            .headers(self.build_headers(None).await)
            .send()
            .await?;
        Self::handle_response(resp).await
    }

    pub async fn invite_family_member(
        &self,
        family_id: Uuid,
        phone: &str,
        role: FamilyRole,
    ) -> Result<(), ApiClientError> {
        let url = format!(
            "{}{}/{}/invites",
            self.base_url,
            ApiEndpoints::FAMILIES,
            family_id
        );
        let resp = self
            .http
            .post(&url)
            .headers(self.build_headers(None).await)
            .json(&serde_json::json!({
                "phone": phone,
                "role": role
            }))
            .send()
            .await?;
        if resp.status().is_success() {
            Ok(())
        } else {
            Self::handle_response::<serde_json::Value>(resp).await?;
            Ok(())
        }
    }

    pub async fn list_pending_invites(&self) -> Result<Vec<PendingFamilyInvite>, ApiClientError> {
        let url = format!("{}/api/v1/families/pending-invites", self.base_url);
        let resp = self
            .http
            .get(&url)
            .headers(self.build_headers(None).await)
            .send()
            .await?;
        Self::handle_response(resp).await
    }

    pub async fn create_family_detailed(
        &self,
        req: &CreateFamilyRequest,
    ) -> Result<CreateFamilyResponse, ApiClientError> {
        let url = format!("{}{}", self.base_url, ApiEndpoints::FAMILIES);
        let resp = self
            .http
            .post(&url)
            .headers(self.build_headers(None).await)
            .json(req)
            .send()
            .await?;
        Self::handle_response(resp).await
    }

    pub async fn create_family(&self, req: &CreateFamilyRequest) -> Result<Family, ApiClientError> {
        let res = self.create_family_detailed(req).await?;
        Ok(res.family)
    }

    pub async fn join_family(&self, invite_code: &str) -> Result<Family, ApiClientError> {
        let res = self
            .join_family_detailed(&JoinFamilyRequest {
                invite_code: Some(invite_code.to_string()),
                invite_id: None,
                display_name: None,
                role: None,
            })
            .await?;
        Ok(res.family)
    }

    pub async fn join_family_detailed(
        &self,
        req: &JoinFamilyRequest,
    ) -> Result<JoinFamilyResponse, ApiClientError> {
        let url = format!("{}{}/join", self.base_url, ApiEndpoints::FAMILIES);
        let resp = self
            .http
            .post(&url)
            .headers(self.build_headers(None).await)
            .json(req)
            .send()
            .await?;
        Self::handle_response(resp).await
    }

    pub async fn join_family_request(
        &self,
        req: &JoinFamilyRequest,
    ) -> Result<Family, ApiClientError> {
        let res = self.join_family_detailed(req).await?;
        Ok(res.family)
    }

    pub async fn update_member_role(
        &self,
        family_id: Uuid,
        member_id: Uuid,
        role: FamilyRole,
    ) -> Result<(), ApiClientError> {
        let url = format!(
            "{}{}/{}/members/{}/role",
            self.base_url,
            ApiEndpoints::FAMILIES,
            family_id,
            member_id
        );
        let role_str = match role {
            FamilyRole::Head => "head",
            FamilyRole::Custodian => "custodian",
            FamilyRole::Member => "member",
        };
        let resp = self
            .http
            .patch(&url)
            .headers(self.build_headers(None).await)
            .json(&serde_json::json!({ "role": role_str }))
            .send()
            .await?;
        if resp.status().is_success() {
            Ok(())
        } else {
            Self::handle_response::<serde_json::Value>(resp).await?;
            Ok(())
        }
    }

    // ================= WALLETS =================

    pub async fn list_wallets_detailed(
        &self,
        scope: &LedgerScope,
    ) -> Result<Vec<CustodyWalletView>, ApiClientError> {
        let mut url = format!("{}{}", self.base_url, ApiEndpoints::WALLETS);
        match scope {
            LedgerScope::Personal { .. } => url.push_str("?scope=personal"),
            LedgerScope::Family { family_id } => {
                url.push_str(&format!("?scope=family&family_id={}", family_id))
            }
        }
        let resp = self
            .http
            .get(&url)
            .headers(self.build_headers(None).await)
            .send()
            .await?;
        Self::handle_response(resp).await
    }

    pub async fn list_wallets(&self, scope: &LedgerScope) -> Result<Vec<Wallet>, ApiClientError> {
        let detailed = self.list_wallets_detailed(scope).await?;
        Ok(detailed.into_iter().map(|v| v.into_wallet().0).collect())
    }

    pub async fn create_wallet(&self, req: &CreateWalletRequest) -> Result<Wallet, ApiClientError> {
        let url = format!("{}{}", self.base_url, ApiEndpoints::WALLETS);
        let resp = self
            .http
            .post(&url)
            .headers(self.build_headers(None).await)
            .json(req)
            .send()
            .await?;
        Self::handle_response(resp).await
    }

    // ================= TRANSACTIONS =================

    pub async fn list_transactions(
        &self,
        scope: &LedgerScope,
    ) -> Result<Vec<Transaction>, ApiClientError> {
        let mut url = format!("{}{}", self.base_url, ApiEndpoints::TRANSACTIONS);
        match scope {
            LedgerScope::Personal { .. } => url.push_str("?scope=personal"),
            LedgerScope::Family { family_id } => {
                url.push_str(&format!("?scope=family&family_id={}", family_id))
            }
        }
        let resp = self
            .http
            .get(&url)
            .headers(self.build_headers(None).await)
            .send()
            .await?;
        Self::handle_response(resp).await
    }

    pub async fn create_transaction(
        &self,
        req: &CreateTransactionRequest,
    ) -> Result<TransactionExecutionResult, ApiClientError> {
        let url = format!("{}{}", self.base_url, ApiEndpoints::TRANSACTIONS);
        let resp = self
            .http
            .post(&url)
            .headers(self.build_headers(req.idempotency_key).await)
            .json(req)
            .send()
            .await?;
        Self::handle_response(resp).await
    }

    // ================= PROJECTS =================

    pub async fn list_projects(
        &self,
        family_id: Option<Uuid>,
    ) -> Result<Vec<Project>, ApiClientError> {
        let mut url = format!("{}{}", self.base_url, ApiEndpoints::PROJECTS);
        if let Some(fid) = family_id {
            url.push_str(&format!("?family_id={}", fid));
        }
        let resp = self
            .http
            .get(&url)
            .headers(self.build_headers(None).await)
            .send()
            .await?;
        Self::handle_response(resp).await
    }

    pub async fn create_project(
        &self,
        req: &CreateProjectRequest,
    ) -> Result<Project, ApiClientError> {
        let url = format!("{}{}", self.base_url, ApiEndpoints::PROJECTS);
        let resp = self
            .http
            .post(&url)
            .headers(self.build_headers(None).await)
            .json(req)
            .send()
            .await?;
        Self::handle_response(resp).await
    }

    // ================= LOANS =================

    pub async fn list_loans(&self, scope: &LedgerScope) -> Result<Vec<Loan>, ApiClientError> {
        let mut url = format!("{}{}", self.base_url, ApiEndpoints::LOANS);
        match scope {
            LedgerScope::Personal { .. } => url.push_str("?scope=personal"),
            LedgerScope::Family { family_id } => {
                url.push_str(&format!("?scope=family&family_id={}", family_id))
            }
        }
        let resp = self
            .http
            .get(&url)
            .headers(self.build_headers(None).await)
            .send()
            .await?;
        Self::handle_response(resp).await
    }

    pub async fn create_loan(&self, req: &CreateLoanRequest) -> Result<Loan, ApiClientError> {
        let url = format!("{}{}", self.base_url, ApiEndpoints::LOANS);
        let resp = self
            .http
            .post(&url)
            .headers(self.build_headers(None).await)
            .json(req)
            .send()
            .await?;
        Self::handle_response(resp).await
    }

    pub async fn repay_loan(
        &self,
        loan_id: Uuid,
        amount: Decimal,
    ) -> Result<serde_json::Value, ApiClientError> {
        let url = format!("{}{}/{}/repay", self.base_url, ApiEndpoints::LOANS, loan_id);
        let resp = self
            .http
            .post(&url)
            .headers(self.build_headers(None).await)
            .json(&serde_json::json!({
                "amount": amount
            }))
            .send()
            .await?;
        Self::handle_response(resp).await
    }

    // ================= PLANS =================

    pub async fn list_plans(&self, scope: &LedgerScope) -> Result<Vec<FuturePlan>, ApiClientError> {
        let mut url = format!("{}{}", self.base_url, ApiEndpoints::PLANS);
        match scope {
            LedgerScope::Personal { .. } => url.push_str("?scope=personal"),
            LedgerScope::Family { family_id } => {
                url.push_str(&format!("?scope=family&family_id={}", family_id))
            }
        }
        let resp = self
            .http
            .get(&url)
            .headers(self.build_headers(None).await)
            .send()
            .await?;
        Self::handle_response(resp).await
    }

    pub async fn list_future_plans(
        &self,
        scope: &LedgerScope,
    ) -> Result<Vec<FuturePlan>, ApiClientError> {
        self.list_plans(scope).await
    }

    pub async fn create_plan(&self, req: &CreatePlanRequest) -> Result<FuturePlan, ApiClientError> {
        let url = format!("{}{}", self.base_url, ApiEndpoints::PLANS);
        let resp = self
            .http
            .post(&url)
            .headers(self.build_headers(None).await)
            .json(req)
            .send()
            .await?;
        Self::handle_response(resp).await
    }

    pub async fn allocate_plan(
        &self,
        plan_id: Uuid,
        amount: Decimal,
    ) -> Result<serde_json::Value, ApiClientError> {
        let url = format!(
            "{}{}/{}/allocate",
            self.base_url,
            ApiEndpoints::PLANS,
            plan_id
        );
        let resp = self
            .http
            .post(&url)
            .headers(self.build_headers(None).await)
            .json(&serde_json::json!({
                "amount": amount
            }))
            .send()
            .await?;
        Self::handle_response(resp).await
    }

    // ================= REPORTS =================

    pub async fn get_mothers_report(
        &self,
        scope: &LedgerScope,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Result<MothersReport, ApiClientError> {
        let mut url = format!(
            "{}{}?start={}&end={}",
            self.base_url,
            ApiEndpoints::REPORTS_SUMMARY,
            start.to_rfc3339(),
            end.to_rfc3339()
        );
        match scope {
            LedgerScope::Personal { .. } => url.push_str("&scope=personal"),
            LedgerScope::Family { family_id } => {
                url.push_str(&format!("&scope=family&family_id={}", family_id))
            }
        }
        let resp = self
            .http
            .get(&url)
            .headers(self.build_headers(None).await)
            .send()
            .await?;
        Self::handle_response(resp).await
    }

    pub async fn create_share_link(
        &self,
        family_id: Uuid,
        locale: Locale,
        valid_days: i64,
    ) -> Result<ShareLinkResponse, ApiClientError> {
        let url = format!("{}/api/v1/reports/share", self.base_url);
        let resp = self
            .http
            .post(&url)
            .headers(self.build_headers(None).await)
            .json(&serde_json::json!({
                "family_id": family_id,
                "locale": locale.as_str(),
                "valid_days": valid_days
            }))
            .send()
            .await?;
        Self::handle_response(resp).await
    }
}
