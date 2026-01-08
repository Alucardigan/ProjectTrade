export interface PendingOrdersResponse {
    orders: Order[];
}

export interface Order {
    order_id: string;
    user_id: string;
    ticker: string;
    quantity: number;
    price_per_share: number;
    order_type: string;
    status: string;
}