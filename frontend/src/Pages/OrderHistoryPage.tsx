import { DashboardNavbar } from "@/components/CustomComponents/DashboardNavbar";
import { Card } from "@/components/retroui/Card";
import { Text } from "@/components/retroui/Text";
import { Badge } from "@/components/retroui/Badge";
import { Button } from "@/components/retroui/Button";
import { Clock, CheckCircle2, XCircle, ArrowLeft } from "lucide-react";
import { useNavigate } from "react-router-dom";

// Mock Data
const pendingOrders = [
    { id: "1", ticker: "NVDA", type: "Buy", quantity: 10, price: 485.09, date: "2024-01-07 10:30 AM" },
    { id: "2", ticker: "TSLA", type: "Sell", quantity: 5, price: 235.40, date: "2024-01-07 11:15 AM" },
];

const executedOrders = [
    { id: "3", ticker: "AAPL", type: "Buy", quantity: 50, price: 185.92, total: 9296.00, date: "2024-01-06 02:20 PM", status: "Executed" },
    { id: "4", ticker: "MSFT", type: "Buy", quantity: 20, price: 370.50, total: 7410.00, date: "2024-01-05 09:45 AM", status: "Executed" },
    { id: "5", ticker: "GOOGL", type: "Sell", quantity: 15, price: 138.20, total: 2073.00, date: "2024-01-04 03:10 PM", status: "Executed" },
    { id: "6", ticker: "AMZN", type: "Buy", quantity: 100, price: 145.20, total: 14520.00, date: "2024-01-03 11:30 AM", status: "Cancelled" },
];

const OrderHistoryPage = () => {
    const navigate = useNavigate();

    return (
        <div className="min-h-screen bg-yellow-50/50 font-sans pb-12">
            <DashboardNavbar />
            <div className="max-w-4xl mx-auto p-6 md:p-12">

                {/* Header */}
                <div className="mb-8 flex flex-col md:flex-row justify-between items-start md:items-center gap-4">
                    <div>
                        <Button
                            variant="ghost"
                            onClick={() => navigate('/portfolio')}
                            className="mb-2 text-gray-600 hover:text-black pl-0"
                        >
                            <ArrowLeft className="w-4 h-4 mr-2" /> Back to Portfolio
                        </Button>
                        <Text as="h1" className="text-4xl font-black text-gray-900 tracking-tight">Order History</Text>
                        <Text className="text-gray-600 font-medium mt-2">View your pending and past transactions.</Text>
                    </div>
                </div>

                {/* Pending Orders Section */}
                <div className="mb-12">
                    <div className="flex items-center gap-2 mb-4">
                        <Clock className="w-6 h-6 text-blue-600" />
                        <Text as="h2" className="text-2xl font-black text-gray-800">Pending Orders</Text>
                        <Badge className="bg-blue-100 text-blue-700 border-blue-200">{pendingOrders.length}</Badge>
                    </div>

                    <div className="space-y-4">
                        {pendingOrders.map((order) => (
                            <Card key={order.id} className="bg-white border-2 border-black shadow-[4px_4px_0px_0px_rgba(0,0,0,1)]">
                                <Card.Content className="p-6 flex flex-col md:flex-row justify-between items-center gap-4">
                                    <div className="flex items-center gap-4 w-full md:w-auto">
                                        <div className={`p-3 rounded-lg border-2 border-black ${order.type === 'Buy' ? 'bg-green-100' : 'bg-red-100'}`}>
                                            <Text className={`font-black uppercase ${order.type === 'Buy' ? 'text-green-700' : 'text-red-700'}`}>{order.type}</Text>
                                        </div>
                                        <div>
                                            <Text className="font-black text-xl">{order.ticker}</Text>
                                            <Text className="text-gray-500 text-sm font-medium">{order.date}</Text>
                                        </div>
                                    </div>

                                    <div className="flex items-center gap-8 w-full md:w-auto justify-between md:justify-end">
                                        <div className="text-right">
                                            <Text className="text-gray-500 text-xs font-bold uppercase">Quantity</Text>
                                            <Text className="font-bold text-lg">{order.quantity}</Text>
                                        </div>
                                        <div className="text-right">
                                            <Text className="text-gray-500 text-xs font-bold uppercase">Est. Price</Text>
                                            <Text className="font-bold text-lg">${order.price.toFixed(2)}</Text>
                                        </div>
                                        <Button size="sm" className="bg-red-500 hover:bg-red-600 text-white border-2 border-black shadow-[2px_2px_0px_0px_rgba(0,0,0,1)] hover:translate-y-[1px] hover:shadow-[1px_1px_0px_0px_rgba(0,0,0,1)]">
                                            Cancel
                                        </Button>
                                    </div>
                                </Card.Content>
                            </Card>
                        ))}
                        {pendingOrders.length === 0 && (
                            <div className="text-center py-8 bg-gray-50 rounded-lg border-2 border-dashed border-gray-300">
                                <Text className="text-gray-500 font-medium">No pending orders</Text>
                            </div>
                        )}
                    </div>
                </div>

                {/* Executed Orders Section */}
                <div>
                    <div className="flex items-center gap-2 mb-4">
                        <CheckCircle2 className="w-6 h-6 text-green-600" />
                        <Text as="h2" className="text-2xl font-black text-gray-800">Order History</Text>
                    </div>

                    <div className="bg-white border-2 border-black shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] rounded-lg overflow-hidden">
                        <div className="overflow-x-auto">
                            <table className="w-full">
                                <thead className="bg-gray-100 border-b-2 border-black">
                                    <tr>
                                        <th className="px-6 py-4 text-left text-sm font-black uppercase tracking-wider text-gray-600">Date</th>
                                        <th className="px-6 py-4 text-left text-sm font-black uppercase tracking-wider text-gray-600">Action</th>
                                        <th className="px-6 py-4 text-left text-sm font-black uppercase tracking-wider text-gray-600">Asset</th>
                                        <th className="px-6 py-4 text-right text-sm font-black uppercase tracking-wider text-gray-600">Quantity</th>
                                        <th className="px-6 py-4 text-right text-sm font-black uppercase tracking-wider text-gray-600">Price</th>
                                        <th className="px-6 py-4 text-right text-sm font-black uppercase tracking-wider text-gray-600">Total</th>
                                        <th className="px-6 py-4 text-center text-sm font-black uppercase tracking-wider text-gray-600">Status</th>
                                    </tr>
                                </thead>
                                <tbody className="divide-y-2 divide-gray-100">
                                    {executedOrders.map((order) => (
                                        <tr key={order.id} className="hover:bg-yellow-50 transition-colors">
                                            <td className="px-6 py-4 whitespace-nowrap">
                                                <Text className="font-medium text-gray-900">{order.date}</Text>
                                            </td>
                                            <td className="px-6 py-4 whitespace-nowrap">
                                                <Badge variant={order.type === 'Buy' ? 'success' : 'destructive'} className="border-black">
                                                    {order.type}
                                                </Badge>
                                            </td>
                                            <td className="px-6 py-4 whitespace-nowrap">
                                                <Text className="font-black text-gray-900">{order.ticker}</Text>
                                            </td>
                                            <td className="px-6 py-4 whitespace-nowrap text-right">
                                                <Text className="font-bold text-gray-700">{order.quantity}</Text>
                                            </td>
                                            <td className="px-6 py-4 whitespace-nowrap text-right">
                                                <Text className="font-bold text-gray-700">${order.price.toFixed(2)}</Text>
                                            </td>
                                            <td className="px-6 py-4 whitespace-nowrap text-right">
                                                <Text className="font-black text-gray-900">${order.total.toLocaleString(undefined, { minimumFractionDigits: 2 })}</Text>
                                            </td>
                                            <td className="px-6 py-4 whitespace-nowrap text-center">
                                                {order.status === 'Executed' ? (
                                                    <div className="flex items-center justify-center gap-1 text-green-600 font-bold text-sm">
                                                        <CheckCircle2 className="w-4 h-4" /> Executed
                                                    </div>
                                                ) : (
                                                    <div className="flex items-center justify-center gap-1 text-gray-400 font-bold text-sm">
                                                        <XCircle className="w-4 h-4" /> Cancelled
                                                    </div>
                                                )}
                                            </td>
                                        </tr>
                                    ))}
                                </tbody>
                            </table>
                        </div>
                    </div>
                </div>

            </div>
        </div>
    );
};

export default OrderHistoryPage;
