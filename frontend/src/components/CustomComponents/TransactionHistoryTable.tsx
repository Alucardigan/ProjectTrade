import { Text } from "@/components/retroui/Text";
import { Badge } from "@/components/retroui/Badge";
import { CheckCircle2 } from "lucide-react";
import type { Transaction } from "@/types/TransactionResponse";

interface TransactionHistoryTableProps {
    transactions: Transaction[];
}

export const TransactionHistoryTable = ({ transactions }: TransactionHistoryTableProps) => {
    return (
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
                                <th className="px-6 py-4 text-left text-sm font-black uppercase tracking-wider text-gray-600">ID</th>
                                <th className="px-6 py-4 text-left text-sm font-black uppercase tracking-wider text-gray-600">Action</th>
                                <th className="px-6 py-4 text-left text-sm font-black uppercase tracking-wider text-gray-600">Asset</th>
                                <th className="px-6 py-4 text-right text-sm font-black uppercase tracking-wider text-gray-600">Quantity</th>
                                <th className="px-6 py-4 text-right text-sm font-black uppercase tracking-wider text-gray-600">Price</th>
                                <th className="px-6 py-4 text-right text-sm font-black uppercase tracking-wider text-gray-600">Total</th>
                                <th className="px-6 py-4 text-center text-sm font-black uppercase tracking-wider text-gray-600">TimeStamp</th>
                            </tr>
                        </thead>
                        <tbody className="divide-y-2 divide-gray-100">
                            {transactions.map((transaction) => {
                                const quantity = Number(transaction.quantity);
                                const price = Number(transaction.price_per_share);
                                const total = quantity * price;
                                return (
                                    <tr key={transaction.transaction_id} className="hover:bg-yellow-50 transition-colors">
                                        <td className="px-6 py-4 whitespace-nowrap">
                                            <Text className="font-medium text-gray-900">{transaction.transaction_id.slice(0, 8)}...</Text>
                                        </td>
                                        <td className="px-6 py-4 whitespace-nowrap">
                                            <Badge variant={transaction.order_type === 'Buy' ? 'success' : 'destructive'} className="border-black">
                                                {transaction.order_type}
                                            </Badge>
                                        </td>
                                        <td className="px-6 py-4 whitespace-nowrap">
                                            <Text className="font-black text-gray-900">{transaction.ticker}</Text>
                                        </td>
                                        <td className="px-6 py-4 whitespace-nowrap text-right">
                                            <Text className="font-bold text-gray-700">{quantity}</Text>
                                        </td>
                                        <td className="px-6 py-4 whitespace-nowrap text-right">
                                            <Text className="font-bold text-gray-700">${price.toFixed(2)}</Text>
                                        </td>
                                        <td className="px-6 py-4 whitespace-nowrap text-right">
                                            <Text className="font-black text-gray-900">${total.toLocaleString(undefined, { minimumFractionDigits: 2 })}</Text>
                                        </td>
                                        <td className="px-6 py-4 whitespace-nowrap text-center">
                                            <Text className="font-black text-gray-900">{transaction.executed_at}</Text>
                                        </td>
                                    </tr>
                                );
                            })}
                        </tbody>
                    </table>
                    {transactions.length === 0 && (
                        <div className="text-center py-8">
                            <Text className="text-gray-500 font-medium">No transaction history</Text>
                        </div>
                    )}
                </div>
            </div>
        </div>
    );
};
