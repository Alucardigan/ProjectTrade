import axios from 'axios';


export enum LoanType {
    Standard = 'Standard',
    Premium = 'Premium',
}

export const requestLoan = async (loanType: LoanType) => {
    const response = await axios.post(`api/loans/${loanType}`, {}, {

    });
    return response.data;
};
