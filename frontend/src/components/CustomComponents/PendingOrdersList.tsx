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

            <div className="bg-white border-2 border-black shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] rounded-lg overflow-hidden">
                <div className="overflow-x-auto">
                    <table className="w-full">
                        <thead className="bg-gray-100 border-b-2 border-black">
                            <tr>
                                <th className="px-6 py-4 text-left text-sm font-black uppercase tracking-wider text-gray-600">ID</th>
                                <th className="px-6 py-4 text-left text-sm font-black uppercase tracking-wider text-gray-600">Action</th>
                                <th className="px-6 py-4 text-left text-sm font-black uppercase tracking-wider text-gray-600">Asset</th>
                                <th className="px-6 py-4 text-right text-sm font-black uppercase tracking-wider text-gray-600">Quantity</th>
                                <th className="px-6 py-4 text-right text-sm font-black uppercase tracking-wider text-gray-600">Est. Price</th>
                                <th className="px-6 py-4 text-center text-sm font-black uppercase tracking-wider text-gray-600">Action</th>
                            </tr>
                        </thead>
                        <tbody className="divide-y-2 divide-gray-100">
                            {orders.map((order) => (
                                <tr key={order.order_id} className="hover:bg-yellow-50 transition-colors">
                                    <td className="px-6 py-4 whitespace-nowrap">
                                        <Text className="font-medium text-gray-900">{order.order_id.slice(0, 8)}...</Text>
                                    </td>
                                    <td className="px-6 py-4 whitespace-nowrap">
                                        <Badge variant={order.order_type === 'Buy' ? 'success' : 'destructive'} className="border-black">
                                            {order.order_type}
                                        </Badge>
                                    </td>
                                    <td className="px-6 py-4 whitespace-nowrap">
                                        <Text className="font-black text-gray-900">{order.ticker}</Text>
                                    </td>
                                    <td className="px-6 py-4 whitespace-nowrap text-right">
                                        <Text className="font-bold text-gray-700">{order.quantity}</Text>
                                    </td>
                                    <td className="px-6 py-4 whitespace-nowrap text-right">
                                        <Text className="font-bold text-gray-700">${Number(order.price_per_share).toFixed(2)}</Text>
                                    </td>
                                    <td className="px-6 py-4 whitespace-nowrap text-center">
                                        <Button size="sm" className="bg-red-500 hover:bg-red-600 text-white border-2 border-black shadow-[2px_2px_0px_0px_rgba(0,0,0,1)] hover:translate-y-[1px] hover:shadow-[1px_1px_0px_0px_rgba(0,0,0,1)]">
                                            Cancel
                                        </Button>
                                    </td>
                                </tr>
                            ))}
                        </tbody>
                    </table>
                    {orders.length === 0 && (
                        <div className="text-center py-8">
                            <Text className="text-gray-500 font-medium">No pending orders</Text>
                        </div>
                    )}
                </div>
            </div>
        </div>
    );
};
