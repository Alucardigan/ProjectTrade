-- Add migration script here
CREATE TABLE transactions (
    transaction_id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    ticker VARCHAR(255) NOT NULL,
    order_type order_type NOT NULL,
    quantity DECIMAL(15, 4) NOT NULL,
    price_per_share DECIMAL(15, 4) NOT NULL,
    executed_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT transaction_quantity_check CHECK (quantity >= 0)
);