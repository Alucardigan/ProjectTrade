import axios from 'axios';
import type { AccountBalanceResponse } from '../types/AccountBalanceResponse';
import type { Transaction } from '@/types/TransactionResponse';
import type { PendingOrdersResponse } from '@/types/PendingOrdersResponse';
export const fetchAccountBalance = async (): Promise<AccountBalanceResponse> => {
    const { data } = await axios.get('/api/account');
    return data;
};

export const fetchTransactionHistory = async (): Promise<Transaction[]> => {
    const { data } = await axios.get('/api/account/transactions');
    console.log(data)
    return data;
};
