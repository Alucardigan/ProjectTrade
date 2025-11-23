-- Add migration script here
CREATE TABLE user_balance (
    uid UUID UNIQUE,
    balance MONEY DEFAULT 0,
    available_balance MONEY DEFAULT 0
);
CREATE INDEX idx_userid on user_balance(uid);