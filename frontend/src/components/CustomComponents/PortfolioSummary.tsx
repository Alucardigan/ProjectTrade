import { Card } from "@/components/retroui/Card";
import { Text } from "@/components/retroui/Text";
import { Badge } from "@/components/retroui/Badge";
import { DollarSign, TrendingUp, TrendingDown, Wallet } from "lucide-react";

interface PortfolioSummaryProps {
    totalValue: number;
    totalGain: number;
    totalCost: number;
    gainPercentage: number;
}

export const PortfolioSummary = ({ totalValue, totalGain, totalCost, gainPercentage }: PortfolioSummaryProps) => {
    return (
        <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
            <Card className="w-full bg-white border-2 border-black shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] hover:translate-y-[2px] hover:shadow-[2px_2px_0px_0px_rgba(0,0,0,1)] transition-all duration-200">
                <Card.Content className="p-6">
                    <div className="flex justify-between items-start">
                        <div>
                            <Text className="text-gray-500 font-bold text-xs uppercase tracking-wider">Total Value</Text>
                            <Text as="h2" className="mt-2 text-3xl font-black text-gray-900">
                                ${totalValue.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })}
                            </Text>
                        </div>
                        <div className="p-3 bg-blue-100 rounded-lg border-2 border-black">
                            <Wallet className="w-6 h-6 text-blue-700" />
                        </div>
                    </div>
                </Card.Content>
            </Card>

            <Card className="w-full bg-white border-2 border-black shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] hover:translate-y-[2px] hover:shadow-[2px_2px_0px_0px_rgba(0,0,0,1)] transition-all duration-200">
                <Card.Content className="p-6">
                    <div className="flex justify-between items-start">
                        <div>
                            <Text className="text-gray-500 font-bold text-xs uppercase tracking-wider">Total Gain/Loss</Text>
                            <div className="flex items-baseline gap-2 mt-2">
                                <Text as="h2" className={`text-3xl font-black ${totalGain >= 0 ? "text-green-600" : "text-red-600"}`}>
                                    {totalGain >= 0 ? "+" : ""}${Math.abs(totalGain).toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })}
                                </Text>
                            </div>
                            <div className="mt-2">
                                <Badge variant={totalGain >= 0 ? "success" : "destructive"} className="border-black border-2">
                                    {totalGain >= 0 ? <TrendingUp className="w-3 h-3 mr-1" /> : <TrendingDown className="w-3 h-3 mr-1" />}
                                    {gainPercentage.toFixed(2)}%
                                </Badge>
                            </div>
                        </div>
                        <div className={`p-3 rounded-lg border-2 border-black ${totalGain >= 0 ? "bg-green-100" : "bg-red-100"}`}>
                            {totalGain >= 0 ? <TrendingUp className="w-6 h-6 text-green-700" /> : <TrendingDown className="w-6 h-6 text-red-700" />}
                        </div>
                    </div>
                </Card.Content>
            </Card>

            <Card className="w-full bg-white border-2 border-black shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] hover:translate-y-[2px] hover:shadow-[2px_2px_0px_0px_rgba(0,0,0,1)] transition-all duration-200">
                <Card.Content className="p-6">
                    <div className="flex justify-between items-start">
                        <div>
                            <Text className="text-gray-500 font-bold text-xs uppercase tracking-wider">Invested Capital</Text>
                            <Text as="h2" className="mt-2 text-3xl font-black text-gray-900">
                                ${totalCost.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })}
                            </Text>
                        </div>
                        <div className="p-3 bg-purple-100 rounded-lg border-2 border-black">
                            <DollarSign className="w-6 h-6 text-purple-700" />
                        </div>
                    </div>
                </Card.Content>
            </Card>
        </div>
    );
};
