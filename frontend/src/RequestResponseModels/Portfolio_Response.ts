export interface PortfolioTicker {
    user_id: string;
    ticker: string;
    quantity: string; // BigDecimal is typically serialized as a string to preserve precision
    total_money_spent: string; // BigDecimal
    total_profit: string; // BigDecimal
    created_at: string; // DateTime<Utc> ISO string
}

export interface PortfolioResponse {
    user_id: string;
    portfolio: PortfolioTicker[];
}
