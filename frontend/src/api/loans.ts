import axios from 'axios';

const API_URL = 'http://localhost:3000';

export enum LoanType {
    Standard = 'Standard',
    Premium = 'Premium',
}

export const requestLoan = async (loanType: LoanType) => {
    const response = await axios.post(`${API_URL}/loans`, { loan_type: loanType }, {
        withCredentials: true,
    });
    return response.data;
};
