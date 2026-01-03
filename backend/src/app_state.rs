use crate::authentication::basic_client::AuthorizationClient;
use crate::models::errors::trade_error::TradeError;
use crate::services::account_management_service::AccountManagementService;
use crate::services::order_management_service::OrderManagementService;
use crate::services::portfolio_management_service::PortfolioManagementService;
use crate::services::ticker_service::TickerService;
use crate::services::trade_service::TradeService;
use crate::services::user_service::UserService;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub user_service: Arc<UserService>,
    pub ticker_service: Arc<TickerService>,
    pub trade_service: Arc<TradeService>,
    pub portfolio_service: Arc<PortfolioManagementService>,
    pub account_management_service: Arc<AccountManagementService>,
    pub order_management_service: Arc<OrderManagementService>,
}

impl AppState {
    pub fn new(db: PgPool, api_key: &str) -> Self {
        let ticker_service = Arc::new(TickerService::new(api_key, db.clone()));
        let account_management_service = Arc::new(AccountManagementService::new(db.clone()));
        let authentication_client = Arc::new(AuthorizationClient::new());
        let portfolio_service = Arc::new(PortfolioManagementService::new(
            db.clone(),
            ticker_service.clone(),
        ));
        let user_service = Arc::new(UserService::new(
            db.clone(),
            account_management_service.clone(),
            portfolio_service.clone(),
            authentication_client.clone(),
        ));

        let order_management_service = Arc::new(OrderManagementService::new(
            db.clone(),
            user_service.clone(),
            ticker_service.clone(),
            account_management_service.clone(),
            portfolio_service.clone(),
        ));

        let trade_service = Arc::new(TradeService::new(
            db.clone(),
            ticker_service.clone(),
            account_management_service.clone(),
            portfolio_service.clone(),
        ));
        tracing::info!("App state created & all services are operational");
        Self {
            db,
            user_service,
            ticker_service,
            trade_service,
            portfolio_service,
            account_management_service,
            order_management_service,
        }
    }
    pub fn start_background_processes(
        &self,
    ) -> Vec<tokio::task::JoinHandle<Result<(), TradeError>>> {
        tracing::info!("Starting background processes");
        vec![self.trade_service.clone().create_order_processor()]
    }
}
