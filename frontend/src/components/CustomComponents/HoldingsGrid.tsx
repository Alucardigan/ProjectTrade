import { Card } from "@/components/retroui/Card";
import { Text } from "@/components/retroui/Text";
import { Badge } from "@/components/retroui/Badge";
import { Button } from "@/components/retroui/Button";
import { ArrowUpRight, ArrowDownRight, Layers, DollarSign } from "lucide-react";
import type { PortfolioTicker } from "../../types/Portfolio_Response";
import { useState } from "react";
import { SellModal } from "./SellModal";

interface HoldingsGridProps {
    portfolio: PortfolioTicker[];
}

export const HoldingsGrid = ({ portfolio }: HoldingsGridProps) => {
    const [selectedSellTicker, setSelectedSellTicker] = useState<{ ticker: string, quantity: number, price: number } | null>(null);

    return (
        <div className="space-y-4">
            <div className="flex items-center gap-2">
                <Layers className="w-5 h-5 text-gray-700" />
                <Text as="h3" className="text-xl font-bold text-gray-800">Holdings</Text>
            </div>
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                {portfolio.map((item) => {
                    const currentValue = Number(item.total_money_spent) + Number(item.total_profit);
                    const profit = Number(item.total_profit);
                    const isProfit = profit >= 0;
                    const price = currentValue / Number(item.quantity);
                    const profitPercent = (profit / Number(item.total_money_spent)) * 100;

                    return (
                        <Card key={item.ticker} className="w-full bg-white border-2 border-black shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] hover:translate-y-[2px] hover:shadow-[2px_2px_0px_0px_rgba(0,0,0,1)] transition-all duration-200 group">
                            <Card.Content className="p-6">
                                <div className="flex justify-between items-start mb-4">
                                    <div className="bg-gray-100 p-3 rounded-xl border-2 border-black group-hover:bg-yellow-100 transition-colors">
                                        <Text as="h4" className="font-black text-xl text-gray-900">{item.ticker}</Text>
                                    </div>
                                    <div className="text-right">
                                        <Badge variant={isProfit ? "success" : "destructive"} className="border-black border-2 mb-1">
                                            {isProfit ? <ArrowUpRight className="w-3 h-3 mr-1" /> : <ArrowDownRight className="w-3 h-3 mr-1" />}
                                            {profitPercent.toFixed(2)}%
                                        </Badge>
                                    </div>
                                </div>

                                <div className="space-y-3 mb-6">
                                    <div className="flex justify-between items-center border-b-2 border-dashed border-gray-200 pb-2">
                                        <Text className="text-gray-500 text-sm font-bold">Quantity</Text>
                                        <Text className="font-bold text-gray-900">{item.quantity}</Text>
                                    </div>
                                    <div className="flex justify-between items-center border-b-2 border-dashed border-gray-200 pb-2">
                                        <Text className="text-gray-500 text-sm font-bold">Current Price</Text>
                                        <Text className="font-bold text-gray-900">${price.toFixed(2)}</Text>
                                    </div>
                                    <div className="flex justify-between items-center pt-1">
                                        <Text className="text-gray-500 text-sm font-bold">Total Value</Text>
                                        <Text className="font-black text-gray-900 text-lg">
                                            ${currentValue.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })}
                                        </Text>
                                    </div>
                                    <div className="flex justify-between items-center pt-1">
                                        <Text className="text-gray-500 text-sm font-bold">Profit/Loss</Text>
                                        <Text className={`font-black text-lg ${isProfit ? "text-green-600" : "text-red-600"}`}>
                                            {isProfit ? "+" : ""}{profit.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })}
                                        </Text>
                                    </div>
                                </div>

                                <Button
                                    onClick={() => setSelectedSellTicker({ ticker: item.ticker, quantity: Number(item.quantity), price })}
                                    className="w-full bg-white hover:bg-red-50 text-red-600 border-2 border-black shadow-[2px_2px_0px_0px_rgba(0,0,0,1)] hover:shadow-[1px_1px_0px_0px_rgba(0,0,0,1)] hover:translate-y-[1px] transition-all font-bold flex items-center justify-center gap-2"
                                >
                                    <DollarSign className="w-4 h-4" /> Sell Asset
                                </Button>
                            </Card.Content>
                        </Card>
                    );
                })}
            </div>

            {selectedSellTicker && (
                <SellModal
                    isOpen={!!selectedSellTicker}
                    onClose={() => setSelectedSellTicker(null)}
                    ticker={selectedSellTicker.ticker}
                    maxQuantity={selectedSellTicker.quantity}
                    currentPrice={selectedSellTicker.price}
                />
            )}
        </div>
    );
};
