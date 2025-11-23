-- Add migration script here
CREATE TABLE portfolio(
    uid UUID UNIQUE NOT NULL,
    ticker VARCHAR NOT NULL,
    quantity DECIMAL NOT NULL,
    purchase MONEY NOT NULL
);
CREATE INDEX idx_userid_portfolio on portfolio(uid);