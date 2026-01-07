-- Add migration script here
CREATE TABLE stock_prices (
    ticker VARCHAR(16) NOT NULL,
    date TIMESTAMPTZ NOT NULL,
    close NUMERIC,
    volume BIGINT,
    open NUMERIC,
    high NUMERIC,
    low NUMERIC,
    PRIMARY KEY (ticker, date)
);