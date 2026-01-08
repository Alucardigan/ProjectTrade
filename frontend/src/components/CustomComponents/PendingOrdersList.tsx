import { Card } from "@/components/retroui/Card";
import { Text } from "@/components/retroui/Text";
import { Button } from "@/components/retroui/Button";
import { Clock } from "lucide-react";
import { Badge } from "@/components/retroui/Badge";
import type { Order } from "@/types/PendingOrdersResponse";

interface PendingOrdersListProps {
    orders: Order[];
}

export const PendingOrdersList = ({ orders }: PendingOrdersListProps) => {
    return (
        <div className="mb-12">
            <div className="flex items-center gap-2 mb-4">
                <Clock className="w-6 h-6 text-blue-600" />
                <Text as="h2" className="text-2xl font-black text-gray-800">Pending Orders</Text>
                <Badge className="bg-blue-100 text-blue-700 border-blue-200">{orders.length}</Badge>
            </div>

            <div className="space-y-4">
                {orders.map((order) => (
                    <Card key={order.order_id} className="bg-white border-2 border-black shadow-[4px_4px_0px_0px_rgba(0,0,0,1)]">
                        <Card.Content className="p-6 flex flex-col md:flex-row justify-between items-center gap-4">
                            <div className="flex items-center gap-4 w-full md:w-auto">
                                <div className={`p-3 rounded-lg border-2 border-black ${order.order_type === 'Buy' ? 'bg-green-100' : 'bg-red-100'}`}>
                                    <Text className={`font-black uppercase ${order.order_type === 'Buy' ? 'text-green-700' : 'text-red-700'}`}>{order.order_type}</Text>
                                </div>
                                <div>
                                    <Text className="font-black text-xl">{order.ticker}</Text>
                                    <Text className="text-gray-500 text-sm font-medium">Order ID: {order.order_id.slice(0, 8)}...</Text>
                                </div>
                            </div>

                            <div className="flex items-center gap-8 w-full md:w-auto justify-between md:justify-end">
                                <div className="text-right">
                                    <Text className="text-gray-500 text-xs font-bold uppercase">Quantity</Text>
                                    <Text className="font-bold text-lg">{order.quantity}</Text>
                                </div>
                                <div className="text-right">
                                    <Text className="text-gray-500 text-xs font-bold uppercase">Est. Price</Text>
                                    <Text className="font-bold text-lg">${order.price_per_share.toFixed(2)}</Text>
                                </div>
                                <Button size="sm" className="bg-red-500 hover:bg-red-600 text-white border-2 border-black shadow-[2px_2px_0px_0px_rgba(0,0,0,1)] hover:translate-y-[1px] hover:shadow-[1px_1px_0px_0px_rgba(0,0,0,1)]">
                                    Cancel
                                </Button>
                            </div>
                        </Card.Content>
                    </Card>
                ))}
                {orders.length === 0 && (
                    <div className="text-center py-8 bg-gray-50 rounded-lg border-2 border-dashed border-gray-300">
                        <Text className="text-gray-500 font-medium">No pending orders</Text>
                    </div>
                )}
            </div>
        </div>
    );
};
