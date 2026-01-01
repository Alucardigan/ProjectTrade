import { Card } from "@/components/retroui/Card";
import { Text } from "@/components/retroui/Text";

interface PortfolioSummaryProps {
    totalValue: number;
    totalGain: number;
    totalCost: number;
    gainPercentage: number;
}

export const PortfolioSummary = ({ totalValue, totalGain, totalCost, gainPercentage }: PortfolioSummaryProps) => {
    return (
        <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
            <Card className="w-full bg-white border-none shadow-lg hover:shadow-xl transition-shadow duration-300">
                <Card.Content className="p-6">
                    <Text className="text-gray-500 font-medium text-sm uppercase tracking-wider">Total Value</Text>
                    <Text as="h2" className="mt-2 text-gray-900">
                        ${totalValue.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })}
                    </Text>
                </Card.Content>
            </Card>

            <Card className="w-full bg-white border-none shadow-lg hover:shadow-xl transition-shadow duration-300">
                <Card.Content className="p-6">
                    <Text className="text-gray-500 font-medium text-sm uppercase tracking-wider">Total Gain/Loss</Text>
                    <div className="flex items-baseline gap-2 mt-2">
                        <Text as="h2" className={totalGain >= 0 ? "text-green-600" : "text-red-600"}>
                            {totalGain >= 0 ? "+" : ""}${Math.abs(totalGain).toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })}
                        </Text>
                        <span className={`text-sm font-bold px-2 py-0.5 rounded-full ${totalGain >= 0 ? "bg-green-100 text-green-700" : "bg-red-100 text-red-700"}`}>
                            {gainPercentage >= 0 ? "+" : ""}{gainPercentage.toFixed(2)}%
                        </span>
                    </div>
                </Card.Content>
            </Card>

            <Card className="w-full bg-white border-none shadow-lg hover:shadow-xl transition-shadow duration-300">
                <Card.Content className="p-6">
                    <Text className="text-gray-500 font-medium text-sm uppercase tracking-wider">Invested Capital</Text>
                    <Text as="h2" className="mt-2 text-gray-900">
                        ${totalCost.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })}
                    </Text>
                </Card.Content>
            </Card>
        </div>
    );
};
