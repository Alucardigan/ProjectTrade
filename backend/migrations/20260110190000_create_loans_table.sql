CREATE TYPE loan_status AS ENUM ('ONGOING', 'ONHOLD', 'PAID', 'FORGIVEN', 'DEFAULTED');

CREATE TABLE loans (
    loan_id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(user_id),
    principal DECIMAL(15, 4) NOT NULL,
    original_loan_amount DECIMAL(15, 4) NOT NULL,
    interest_rate DECIMAL(5, 4) NOT NULL,
    status loan_status NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_paid_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
