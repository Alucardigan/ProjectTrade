import { Card } from "@/components/retroui/Card";
import { Text } from "@/components/retroui/Text";
import type { PortfolioTicker } from "../../RequestResponseModels/Portfolio_Response";

interface HoldingsGridProps {
    portfolio: PortfolioTicker[];
}

export const HoldingsGrid = ({ portfolio }: HoldingsGridProps) => {
    return (
        <div className="space-y-4">
            <Text as="h3" className="text-gray-800">Holdings</Text>
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                {portfolio.map((item) => {
                    const currentValue = Number(item.total_money_spent) + Number(item.total_profit);
                    const profit = Number(item.total_profit);
                    const isProfit = profit >= 0;
                    const price = currentValue / Number(item.quantity);

                    return (
                        <Card key={item.ticker} className="w-full bg-white border border-gray-100 shadow-md hover:shadow-xl hover:-translate-y-1 transition-all duration-300 group">
                            <Card.Content className="p-6">
                                <div className="flex justify-between items-start mb-4">
                                    <div className="bg-gray-100 p-3 rounded-xl group-hover:bg-blue-50 transition-colors">
                                        <Text as="h4" className="font-bold text-gray-900">{item.ticker}</Text>
                                    </div>
                                    <div className={`text-right ${isProfit ? "text-green-600" : "text-red-600"}`}>
                                        <Text className="font-bold text-lg">
                                            {isProfit ? "+" : ""}{profit.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })}
                                        </Text>
                                        <Text className="text-xs font-medium opacity-80">Profit/Loss</Text>
                                    </div>
                                </div>

                                <div className="space-y-3">
                                    <div className="flex justify-between items-center border-b border-gray-50 pb-2">
                                        <Text className="text-gray-500 text-sm">Quantity</Text>
                                        <Text className="font-medium text-gray-900">{item.quantity}</Text>
                                    </div>
                                    <div className="flex justify-between items-center border-b border-gray-50 pb-2">
                                        <Text className="text-gray-500 text-sm">Current Price</Text>
                                        <Text className="font-medium text-gray-900">${price.toFixed(2)}</Text>
                                    </div>
                                    <div className="flex justify-between items-center pt-1">
                                        <Text className="text-gray-500 text-sm">Total Value</Text>
                                        <Text className="font-bold text-gray-900 text-lg">
                                            ${currentValue.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })}
                                        </Text>
                                    </div>
                                </div>
                            </Card.Content>
                        </Card>
                    );
                })}
            </div>
        </div>
    );
};
