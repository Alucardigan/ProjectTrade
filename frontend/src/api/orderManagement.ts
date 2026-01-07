import axios from 'axios';
import type { PlaceOrderRequest } from '../types/PlaceOrderRequest';

export const placeOrder = async (order: PlaceOrderRequest): Promise<void> => {
    await axios.post('/api/orders', order);
};
