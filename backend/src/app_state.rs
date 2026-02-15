use crate::authentication::basic_client::AuthorizationClient;
use crate::models::errors::trade_error::TradeError;
use crate::services::account_management_service::AccountManagementService;
use crate::services::loan_service::LoanService;
use crate::services::market_maker_service;
use crate::services::order_management_service::OrderManagementService;
use crate::services::order_matchbook_service::{self, OrderMatchbookService};
use crate::services::portfolio_management_service::PortfolioManagementService;
use crate::services::ticker_service::TickerService;
use crate::services::trade_service::TradeService;
use crate::services::user_service::UserService;
use sqlx::PgPool;
use std::sync::Arc;
use tracing::info;
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    system_user_id: Uuid,
    pub db: PgPool,
    pub user_service: Arc<UserService>,
    pub ticker_service: Arc<TickerService>,
    pub trade_service: Arc<TradeService>,
    pub order_matchbook_service: Arc<OrderMatchbookService>,
    pub portfolio_service: Arc<PortfolioManagementService>,
    pub account_management_service: Arc<AccountManagementService>,
    pub order_management_service: Arc<OrderManagementService>,
    pub loan_service: Arc<LoanService>,
    pub market_maker_service: Arc<market_maker_service::MarketMakerService>,
}

impl AppState {
    pub fn new(db: PgPool, api_key: &str, system_user_id: Uuid) -> Self {
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
        let trade_service = Arc::new(TradeService::new(
            db.clone(),
            ticker_service.clone(),
            account_management_service.clone(),
            portfolio_service.clone(),
        ));
        let order_matchbook_service =
            Arc::new(order_matchbook_service::OrderMatchbookService::new(
                db.clone(),
                trade_service.clone(),
                ticker_service.clone(),
            ));

        let order_management_service = Arc::new(OrderManagementService::new(
            db.clone(),
            user_service.clone(),
            ticker_service.clone(),
            account_management_service.clone(),
            portfolio_service.clone(),
            order_matchbook_service.clone(),
        ));
        let market_maker_service = Arc::new(market_maker_service::MarketMakerService::new(
            db.clone(),
            ticker_service.clone(),
            order_matchbook_service.clone(),
            order_management_service.clone(),
            vec!["AAPL".to_string(), "GOOGL".to_string(), "MSFT".to_string()],
            system_user_id,
        ));

        let loan_service = Arc::new(LoanService::new(
            db.clone(),
            account_management_service.clone(),
        ));
        tracing::info!("App state created & all services are operational");
        Self {
            system_user_id,
            db,
            user_service,
            ticker_service,
            trade_service,
            portfolio_service,
            account_management_service,
            order_management_service,
            loan_service,
            order_matchbook_service,
            market_maker_service,
        }
    }
    pub async fn start_background_processes(
        &self,
    ) -> Vec<tokio::task::JoinHandle<Result<(), TradeError>>> {
        tracing::info!("Starting background processes");
        let mut handles: Vec<tokio::task::JoinHandle<Result<(), TradeError>>> = Vec::new();
        let ticker_ids = vec!["AAPL".to_string(), "GOOGL".to_string(), "MSFT".to_string()];
        info!("Ticker ids: {:?}", ticker_ids);
        self.user_service
            .create_system_user(self.system_user_id, ticker_ids)
            .await;
        self.order_matchbook_service.initialise_orderbooks().await;
        self.market_maker_service.initialise_market().await;

        handles.push(self.order_matchbook_service.create_worker_thread());
        handles.push(self.market_maker_service.spawn_worker_thread().await);
        return handles;
    }
}
