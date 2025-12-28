-- Add migration script here
CREATE TABLE portfolio(
    portfolio_id UUID PRIMARY KEY NOT NULL,
    user_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    ticker VARCHAR NOT NULL,
    quantity DECIMAL NOT NULL,
    total_money_spent DECIMAL(15, 4) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT portfolio_quantity_check CHECK (quantity > 0),
    CONSTRAINT unique_user_ticker UNIQUE (user_id, ticker)
);
CREATE INDEX idx_userid_portfolio on portfolio(user_id);