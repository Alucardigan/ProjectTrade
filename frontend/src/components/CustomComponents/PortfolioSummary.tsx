import { Card } from "@/components/retroui/Card";
import { Text } from "@/components/retroui/Text";
import { Badge } from "@/components/retroui/Badge";
import { DollarSign, TrendingUp, TrendingDown, Wallet, CreditCard } from "lucide-react";

interface PortfolioSummaryProps {
    totalValue: number;
    totalGain: number;
    totalCost: number;
    gainPercentage: number;
    totalLiabilities: number;
}

const formatCompact = (num: number) => {
    return Intl.NumberFormat('en-US', {
        notation: "compact",
        maximumFractionDigits: 2
    }).format(num);
};

export const PortfolioSummary = ({ totalValue, totalGain, totalCost, gainPercentage, totalLiabilities }: PortfolioSummaryProps) => {
    return (
        <div className="grid grid-cols-1 sm:grid-cols-2 xl:grid-cols-1 gap-6">
            <Card className="w-full bg-white border-2 border-black shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] hover:translate-y-[2px] hover:shadow-[2px_2px_0px_0px_rgba(0,0,0,1)] transition-all duration-200">
                <Card.Content className="p-6">
                    <div className="flex justify-between items-center gap-4 h-full">
                        <div className="min-w-0 flex-1 flex flex-col items-center">
                            <Text className="text-gray-500 font-bold text-xs uppercase tracking-wider truncate text-center">Total Value</Text>
                            <Text as="h2" className="mt-2 text-2xl xl:text-3xl font-black text-gray-900 truncate text-center" title={totalValue.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })}>
                                {formatCompact(totalValue)}
                            </Text>
                        </div>
                        <div className="p-3 bg-blue-100 rounded-lg border-2 border-black shrink-0">
                            <Wallet className="w-6 h-6 text-blue-700" />
                        </div>
                    </div>
                </Card.Content>
            </Card>

            <Card className="w-full bg-white border-2 border-black shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] hover:translate-y-[2px] hover:shadow-[2px_2px_0px_0px_rgba(0,0,0,1)] transition-all duration-200">
                <Card.Content className="p-6">
                    <div className="flex justify-between items-center gap-4 h-full">
                        <div className="min-w-0 flex-1 flex flex-col items-center">
                            <Text className="text-gray-500 font-bold text-xs uppercase tracking-wider truncate text-center">Total Gain/Loss</Text>
                            <div className="flex items-baseline justify-center gap-2 mt-2 w-full">
                                <Text as="h2" className={`text-2xl xl:text-3xl font-black truncate text-center ${totalGain >= 0 ? "text-green-600" : "text-red-600"}`} title={`${totalGain >= 0 ? "+" : ""}${Math.abs(totalGain).toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })}`}>
                                    {totalGain >= 0 ? "+" : ""}{formatCompact(Math.abs(totalGain))}
                                </Text>
                            </div>
                            <div className="mt-2 flex justify-center w-full">
                                <Badge variant={totalGain >= 0 ? "success" : "destructive"} className="border-black border-2">
                                    {totalGain >= 0 ? <TrendingUp className="w-3 h-3 mr-1 shrink-0" /> : <TrendingDown className="w-3 h-3 mr-1 shrink-0" />}
                                    <span className="truncate">{gainPercentage.toFixed(2)}%</span>
                                </Badge>
                            </div>
                        </div>
                        <div className={`p-3 rounded-lg border-2 border-black shrink-0 ${totalGain >= 0 ? "bg-green-100" : "bg-red-100"}`}>
                            {totalGain >= 0 ? <TrendingUp className="w-6 h-6 text-green-700" /> : <TrendingDown className="w-6 h-6 text-red-700" />}
                        </div>
                    </div>
                </Card.Content>
            </Card>

            <Card className="w-full bg-white border-2 border-black shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] hover:translate-y-[2px] hover:shadow-[2px_2px_0px_0px_rgba(0,0,0,1)] transition-all duration-200">
                <Card.Content className="p-6">
                    <div className="flex justify-between items-center gap-4 h-full">
                        <div className="min-w-0 flex-1 flex flex-col items-center">
                            <Text className="text-gray-500 font-bold text-xs uppercase tracking-wider truncate text-center">Invested Capital</Text>
                            <Text as="h2" className="mt-2 text-2xl xl:text-3xl font-black text-gray-900 truncate text-center" title={totalCost.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })}>
                                {formatCompact(totalCost)}
                            </Text>
                        </div>
                        <div className="p-3 bg-purple-100 rounded-lg border-2 border-black shrink-0">
                            <DollarSign className="w-6 h-6 text-purple-700" />
                        </div>
                    </div>
                </Card.Content>
            </Card>

            <Card className="w-full bg-white border-2 border-black shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] hover:translate-y-[2px] hover:shadow-[2px_2px_0px_0px_rgba(0,0,0,1)] transition-all duration-200">
                <Card.Content className="p-6">
                    <div className="flex justify-between items-center gap-4 h-full">
                        <div className="min-w-0 flex-1 flex flex-col items-center">
                            <Text className="text-gray-500 font-bold text-xs uppercase tracking-wider truncate text-center">Total Liabilities</Text>
                            <Text as="h2" className="mt-2 text-2xl xl:text-3xl font-black text-red-600 truncate text-center" title={totalLiabilities.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })}>
                                {formatCompact(totalLiabilities)}
                            </Text>
                        </div>
                        <div className="p-3 bg-red-100 rounded-lg border-2 border-black shrink-0">
                            <CreditCard className="w-6 h-6 text-red-700" />
                        </div>
                    </div>
                </Card.Content>
            </Card>
        </div>
    );
};
