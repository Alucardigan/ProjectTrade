export interface Loan {
    loan_id: string;
    user_id: string;
    principal: string; // BigDecimal comes as string usually
    interest_rate: string;
    status: 'ONGOING' | 'ONHOLD' | 'PAID' | 'FORGIVEN' | 'DEFAULTED';
    created_at: string;
    last_paid_at: string;
}
