import { Text } from "@/components/retroui/Text";
import { PortfolioSummary } from "@/components/CustomComponents/PortfolioSummary";
import { HoldingsGrid } from "@/components/CustomComponents/HoldingsGrid";
import type { PortfolioResponse } from "../RequestResponseModels/Portfolio_Response";
import { useMemo } from "react";

// Mock Data
const MOCK_PORTFOLIO: PortfolioResponse = {
    user_id: "user-123",
    portfolio: [
        {
            user_id: "user-123",
            ticker: "AAPL",
            quantity: "15.5",
            total_money_spent: "2325.00", // Avg $150
            total_profit: "450.50",
            created_at: new Date().toISOString()
        },
        {
            user_id: "user-123",
            ticker: "TSLA",
            quantity: "10",
            total_money_spent: "2500.00", // Avg $250
            total_profit: "-320.00",
            created_at: new Date().toISOString()
        },
        {
            user_id: "user-123",
            ticker: "NVDA",
            quantity: "5",
            total_money_spent: "2000.00", // Avg $400
            total_profit: "1200.00",
            created_at: new Date().toISOString()
        },
        {
            user_id: "user-123",
            ticker: "MSFT",
            quantity: "8",
            total_money_spent: "2400.00", // Avg $300
            total_profit: "160.00",
            created_at: new Date().toISOString()
        }
    ]
};

const PortfolioPage = () => {
    // Use mock data directly
    const portfolioResponse = MOCK_PORTFOLIO;

    const totalValue = useMemo(() => {
        return portfolioResponse.portfolio.reduce((acc, item) =>
            acc + Number(item.total_money_spent) + Number(item.total_profit), 0);
    }, [portfolioResponse]);

    const totalGain = useMemo(() => {
        return portfolioResponse.portfolio.reduce((acc, item) =>
            acc + Number(item.total_profit), 0);
    }, [portfolioResponse]);

    const totalCost = useMemo(() => {
        return portfolioResponse.portfolio.reduce((acc, item) =>
            acc + Number(item.total_money_spent), 0);
    }, [portfolioResponse]);

    const gainPercentage = totalCost > 0 ? (totalGain / totalCost) * 100 : 0;

    return (
        <div className="min-h-screen bg-gray-50 p-6 md:p-12 font-sans">
            <div className="max-w-6xl mx-auto space-y-8">

                {/* Header Section */}
                <div className="flex flex-col md:flex-row justify-between items-start md:items-center gap-4">
                    <div>
                        <Text as="h1" className="text-gray-900 tracking-tight">My Portfolio</Text>
                        <Text className="text-gray-500 mt-1">Track your market performance</Text>
                    </div>
                    <div className="bg-white px-4 py-2 rounded-lg border border-gray-200 shadow-sm">
                        <Text className="text-sm text-gray-500 font-medium">Last Updated</Text>
                        <Text className="text-gray-900 font-mono text-sm">{new Date().toLocaleTimeString()}</Text>
                    </div>
                </div>

                {/* Summary Cards */}
                <PortfolioSummary
                    totalValue={totalValue}
                    totalGain={totalGain}
                    totalCost={totalCost}
                    gainPercentage={gainPercentage}
                />

                {/* Holdings List */}
                <HoldingsGrid portfolio={portfolioResponse.portfolio} />
            </div>
        </div>
    );
}

export default PortfolioPage;