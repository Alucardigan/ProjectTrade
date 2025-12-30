-- Add migration script here
create type order_type as enum('BUY', 'SELL');
create type order_status as enum('PENDING', 'RESERVED', 'EXECUTED', 'CANCELLED');
CREATE TABLE orders (
    order_id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(user_id) ON DELETE RESTRICT,
    ticker VARCHAR(10) NOT NULL,
    quantity DECIMAL(15, 4) NOT NULL,
    price_per_share DECIMAL(15, 4) NOT NULL,
    order_type order_type NOT NULL,
    status order_status NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT order_quantity_check CHECK (quantity >= 0)
);

CREATE INDEX idx_orders_user_id ON orders(user_id);
CREATE INDEX idx_orders_status ON orders(status);
CREATE INDEX idx_orders_created_at ON orders(created_at DESC);