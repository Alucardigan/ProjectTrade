import { OrderType } from "./OrderType";

export interface PlaceOrderRequest {
    symbol: string;
    quantity: number;
    order_type: OrderType;
    price_buffer: number;
}
