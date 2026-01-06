import axios from 'axios';
import type { AccountBalanceResponse } from '../types/AccountBalanceResponse';

export const fetchAccountBalance = async (): Promise<AccountBalanceResponse> => {
    const { data } = await axios.get('/api/account');
    return data;
};
