import axios from 'axios';
import type { PlaceOrderRequest } from '../types/PlaceOrderRequest';

export const placeOrder = async (PlaceOrderRequest: PlaceOrderRequest): Promise<any> => {
    console.log("Placing order...");
    try {
        const { data } = await axios.post('/api/orders', PlaceOrderRequest);
        console.log("Order data received:", data);
        return data;
    } catch (error) {
        console.error("Error placing order:", error);
        throw error;
    }
};
