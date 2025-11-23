use uuid::Uuid;

#[allow(dead_code)]
pub struct Order {
    pub order_id: Uuid,
    pub user_id: Uuid,
    pub symbol: String,
    pub quantity: f64,
    pub price: f64,
    pub order_type: OrderType, // Buy or Sell
    pub status: OrderStatus,
}
#[allow(dead_code)]
pub enum OrderType {
    Buy,
    Sell,
}

#[allow(dead_code)]
pub enum OrderStatus {
    Pending,
    Reserved,
    Executed,
    Cancelled,
}
