use std::sync::Arc;

use crate::authentication::basic_client::Auth0LoginResponse;
use crate::authentication::basic_client::AuthorizationClient;
use crate::models::errors::trade_error::TradeError;
use crate::models::errors::user_error::UserError;
use crate::services::account_management_service::AccountManagementService;
use crate::services::portfolio_management_service::PortfolioManagementService;
use bigdecimal::BigDecimal;
use num_traits::Zero;
use oauth2::TokenResponse;
use oauth2::{CsrfToken, PkceCodeVerifier};
use sqlx::PgPool;
use sqlx::Row;
use tracing::info;
use uuid::Uuid;
//current implementation assumes no Errors
#[allow(dead_code)]
#[derive(Clone)]
pub struct UserService {
    user_db: PgPool,
    account_management_service: Arc<AccountManagementService>,
    portfolio_management_service: Arc<PortfolioManagementService>,
    authentication_client: Arc<AuthorizationClient>,
}
#[allow(dead_code)]
impl UserService {
    //need to implement getters
    const SESSION_LENGTH: i64 = 30;
    pub fn new(
        db: PgPool,
        account_management_service: Arc<AccountManagementService>,
        portfolio_management_service: Arc<PortfolioManagementService>,
        authentication_client: Arc<AuthorizationClient>,
    ) -> Self {
        Self {
            user_db: db,
            account_management_service,
            portfolio_management_service,
            authentication_client,
        }
    }

    #[tracing::instrument(skip(self))]
    pub async fn upsert_user(
        &self,
        user_id: Uuid,
        auth0_id: &str,
        username: &str,
        email: &str,
    ) -> Result<Uuid, UserError> {
        let row = sqlx::query(
            "
            INSERT INTO users (user_id, auth_user_id, username, email) 
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (auth_user_id) DO UPDATE SET username = EXCLUDED.username, email = EXCLUDED.email
            RETURNING user_id
            ",
        )
        .bind(user_id)
        .bind(auth0_id)
        .bind(username)
        .bind(email)
        .fetch_one(&self.user_db)
        .await?;

        let inserted_user_id: Uuid = row
            .try_get("user_id")
            .map_err(|e| UserError::DatabaseError(e))?;
        Ok(inserted_user_id)
    }

    pub async fn create_system_user(
        &self,
        system_user_id: Uuid,
        ticker_ids: Vec<String>,
    ) -> Result<Uuid, TradeError> {
        info!("Deleting system user with user_id: {}", system_user_id);
        sqlx::query("DELETE FROM users WHERE user_id = $1")
            .bind(system_user_id)
            .execute(&self.user_db)
            .await
            .map_err(|e| UserError::DatabaseError(e))?;
        self.upsert_user(system_user_id, "system", "system", "system@system.com")
            .await?;
        info!("System user created with user_id: {}", system_user_id);
        self.account_management_service
            .add_user_balance(system_user_id, &BigDecimal::from(100000000))
            .await?;
        info!(
            "System user balance created with user_id: {}",
            system_user_id
        );
        for ticker_id in ticker_ids {
            info!("TICKER: {}", ticker_id);
            self.portfolio_management_service
                .add_to_portfolio(
                    system_user_id,
                    &ticker_id,
                    &BigDecimal::from(100000000),
                    &BigDecimal::zero(),
                )
                .await?;
        }
        Ok(system_user_id)
    }
    //should this function exist?
    #[tracing::instrument(skip(self))]
    pub async fn get_user_uuid(&self, username: &str) -> Result<Uuid, UserError> {
        let rec = sqlx::query("SELECT user_id FROM users WHERE username = $1")
            .bind(username)
            .fetch_one(&self.user_db)
            .await
            .map_err(|e| UserError::DatabaseError(e))?;
        Ok(rec.try_get("user_id")?)
    }

    #[tracing::instrument(skip(self))]
    pub async fn login_user(&self) -> Result<Auth0LoginResponse, UserError> {
        let auth0_login_response = self.authentication_client.auth0_login().await;
        Ok(auth0_login_response)
    }
    #[tracing::instrument(skip(self))]
    pub async fn login_callback(
        &self,
        code: String,
        state_token: String,
        pkce_code_verifier: PkceCodeVerifier,
        csrf_token: CsrfToken,
    ) -> Result<Uuid, UserError> {
        let token_response = self
            .authentication_client
            .auth0_callback(code, state_token, pkce_code_verifier, csrf_token)
            .await?;
        let access_token = token_response.access_token();
        let user_info = self
            .authentication_client
            .get_user_info(access_token)
            .await?;

        // Use email as username if name is missing
        let username = user_info.name.as_deref().unwrap_or(&user_info.email);
        let mut user_id = Uuid::new_v4();
        user_id = self
            .upsert_user(user_id, &user_info.sub, username, &user_info.email)
            .await?;
        //session creation
        let session_id = Uuid::new_v4();
        let expires_at = chrono::Utc::now() + chrono::Duration::days(Self::SESSION_LENGTH);
        sqlx::query(
            "
            INSERT INTO sessions (session_id, user_id, expires_at) 
            VALUES ($1, $2, $3)
            ",
        )
        .bind(session_id)
        .bind(user_id)
        .bind(expires_at)
        .execute(&self.user_db)
        .await
        .map_err(|e| UserError::DatabaseError(e))?;
        Ok(session_id)
    }
}
