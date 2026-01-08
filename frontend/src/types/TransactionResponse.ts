

export interface Transaction {
    transaction_id: string;
    user_id: string;
    ticker: string;
    quantity: number;
    price_per_share: number;
    order_type: string;
    executed_at: string;
}