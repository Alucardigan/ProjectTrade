import axios from 'axios';
import type { Loan } from '../types/Loan';


export enum LoanType {
    Standard = 'Standard',
    Premium = 'Premium',
}

export const requestLoan = async (loanType: LoanType) => {
    const response = await axios.post(`api/loans/${loanType}`, {}, {

    });
    return response.data;
};

export const getLoan = async (): Promise<Loan> => {
    const response = await axios.get(`api/loans`);
    return response.data;
};

export const repayLoan = async (amount: number) => {
    const response = await axios.post(`api/loans/repay`, { amount });
    return response.data;
};
