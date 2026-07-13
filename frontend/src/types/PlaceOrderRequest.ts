import { OrderType } from "./OrderType";

export interface PlaceOrderRequest {
    ticker: string;
    quantity: number;
    order_type: OrderType;
    price_buffer: number;
}
