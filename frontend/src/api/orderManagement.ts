import axios from 'axios';
import type { PlaceOrderRequest } from '../types/PlaceOrderRequest';
import type { Order } from '@/types/PendingOrdersResponse';

export const placeOrder = async (order: PlaceOrderRequest): Promise<void> => {
    await axios.post('/api/orders', order);
};

export const fetchOrderHistory = async (): Promise<Order[]> => {
    const { data } = await axios.get('/api/orders');
    return data;
};