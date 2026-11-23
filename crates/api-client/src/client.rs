use reqwest::Client;
use reqwest::header::{ACCEPT_LANGUAGE, AUTHORIZATION, HeaderMap, HeaderValue};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;
use umuryango_core_domain::models::{
    Family, FamilyRole, FuturePlan, Loan, Project, Transaction, TransactionType, Wallet,
    WalletChannel,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub user_id: Uuid,
    pub email: String,
    pub full_name: String,
    pub locale: Locale,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub full_name: String,
    pub phone_number: Option<String>,
    pub preferred_locale: Option<Locale>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFamilyRequest {
    pub name: String,
    pub currency: Option<String>,
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
    pub tx_date: Option<chrono::DateTime<chrono::Utc>>,
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
    pub interest_rate: Decimal,
    pub is_borrowed: bool,
    pub due_date: Option<chrono::NaiveDate>,
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
    pub share_url: String,
    pub magic_token: String,
}

#[derive(Clone)]
pub struct ApiClient {
    pub base_url: String,
    pub ws_url: String,
    http: Client,
    pub auth_token: Arc<RwLock<Option<String>>>,
    pub locale: Arc<RwLock<Locale>>,
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
        Self::handle_response(resp).await
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

    pub async fn list_families(&self) -> Result<Vec<Family>, ApiClientError> {
        let url = format!("{}{}", self.base_url, ApiEndpoints::FAMILIES);
        let resp = self
            .http
            .get(&url)
            .headers(self.build_headers(None).await)
            .send()
            .await?;
        Self::handle_response(resp).await
    }

    pub async fn create_family(&self, req: &CreateFamilyRequest) -> Result<Family, ApiClientError> {
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

    pub async fn join_family(&self, invite_code: &str) -> Result<Family, ApiClientError> {
        let url = format!("{}{}/join", self.base_url, ApiEndpoints::FAMILIES);
        let resp = self
            .http
            .post(&url)
            .headers(self.build_headers(None).await)
            .json(&serde_json::json!({ "invite_code": invite_code }))
            .send()
            .await?;
        Self::handle_response(resp).await
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

    pub async fn list_wallets(&self, scope: &LedgerScope) -> Result<Vec<Wallet>, ApiClientError> {
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

    pub async fn create_transaction(
        &self,
        req: &CreateTransactionRequest,
        idempotency_key: Option<Uuid>,
    ) -> Result<TransactionExecutionResult, ApiClientError> {
        let url = format!("{}{}", self.base_url, ApiEndpoints::TRANSACTIONS);
        let resp = self
            .http
            .post(&url)
            .headers(self.build_headers(idempotency_key).await)
            .json(req)
            .send()
            .await?;
        Self::handle_response(resp).await
    }

    pub async fn list_transactions(
        &self,
        scope: &LedgerScope,
        wallet_id: Option<Uuid>,
    ) -> Result<Vec<Transaction>, ApiClientError> {
        let mut url = format!("{}{}", self.base_url, ApiEndpoints::TRANSACTIONS);
        match scope {
            LedgerScope::Personal { .. } => url.push_str("?scope=personal"),
            LedgerScope::Family { family_id } => {
                url.push_str(&format!("?scope=family&family_id={}", family_id))
            }
        }
        if let Some(wid) = wallet_id {
            url.push_str(&format!("&wallet_id={}", wid));
        }
        let resp = self
            .http
            .get(&url)
            .headers(self.build_headers(None).await)
            .send()
            .await?;
        Self::handle_response(resp).await
    }

    // ================= PROJECTS =================

    pub async fn list_projects(&self, family_id: Uuid) -> Result<Vec<Project>, ApiClientError> {
        let url = format!(
            "{}{}?family_id={}",
            self.base_url,
            ApiEndpoints::PROJECTS,
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
        let mut url = format!("{}/api/v1/loans", self.base_url);
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

    pub async fn repay_loan(&self, loan_id: Uuid, amount: Decimal) -> Result<(), ApiClientError> {
        let url = format!("{}/api/v1/loans/{}/repay", self.base_url, loan_id);
        let resp = self
            .http
            .post(&url)
            .headers(self.build_headers(None).await)
            .json(&serde_json::json!({ "amount": amount }))
            .send()
            .await?;
        if resp.status().is_success() {
            Ok(())
        } else {
            Self::handle_response::<serde_json::Value>(resp).await?;
            Ok(())
        }
    }

    // ================= FUTURE PLANS =================

    pub async fn list_future_plans(
        &self,
        scope: &LedgerScope,
    ) -> Result<Vec<FuturePlan>, ApiClientError> {
        let mut url = format!("{}/api/v1/plans", self.base_url);
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

    pub async fn allocate_plan(
        &self,
        plan_id: Uuid,
        amount: Decimal,
    ) -> Result<(), ApiClientError> {
        let url = format!("{}/api/v1/plans/{}/allocate", self.base_url, plan_id);
        let resp = self
            .http
            .post(&url)
            .headers(self.build_headers(None).await)
            .json(&serde_json::json!({ "amount": amount }))
            .send()
            .await?;
        if resp.status().is_success() {
            Ok(())
        } else {
            Self::handle_response::<serde_json::Value>(resp).await?;
            Ok(())
        }
    }

    // ================= REPORTS =================

    pub async fn get_report_summary(
        &self,
        scope: &LedgerScope,
        start: chrono::DateTime<chrono::Utc>,
        end: chrono::DateTime<chrono::Utc>,
    ) -> Result<MothersReport, ApiClientError> {
        let mut url = format!(
            "{}{}?from={}&to={}",
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
        expires_days: i64,
    ) -> Result<ShareLinkResponse, ApiClientError> {
        let url = format!("{}/api/v1/reports/share", self.base_url);
        let resp = self
            .http
            .post(&url)
            .headers(self.build_headers(None).await)
            .json(&serde_json::json!({
                "family_id": family_id,
                "expires_days": expires_days
            }))
            .send()
            .await?;
        Self::handle_response(resp).await
    }
}
