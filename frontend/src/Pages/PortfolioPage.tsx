import { Text } from "@/components/retroui/Text";
import { PortfolioSummary } from "@/components/CustomComponents/PortfolioSummary";
import { HoldingsGrid } from "@/components/CustomComponents/HoldingsGrid";
import type { PortfolioResponse } from "../RequestResponseModels/Portfolio_Response";
import { useMemo } from "react";
import { Button } from "@/components/retroui/Button";
import { RefreshCw, Plus, Minus } from "lucide-react";
import { DashboardNavbar } from "@/components/CustomComponents/DashboardNavbar";

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
        <div className="min-h-screen bg-yellow-50/50 font-sans">
            <DashboardNavbar />
            <div className="max-w-6xl mx-auto space-y-8 p-6 md:p-12">

                {/* Header Section */}
                <div className="flex flex-col md:flex-row justify-between items-start md:items-center gap-4 border-b-4 border-black pb-6">
                    <div>
                        <Text as="h1" className="text-5xl font-black text-gray-900 tracking-tight">My Portfolio</Text>
                        <Text className="text-gray-600 font-medium mt-2 text-lg">Track your market performance</Text>
                    </div>
                    <div className="flex flex-col sm:flex-row items-center gap-4">
                        <div className="flex gap-3">
                            <Button
                                onClick={() => window.location.href = '/buy'}
                                className="bg-green-500 hover:bg-green-600 text-white border-2 border-black shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] hover:shadow-[2px_2px_0px_0px_rgba(0,0,0,1)] hover:translate-y-[2px] transition-all font-bold"
                            >
                                <Plus className="w-5 h-5 mr-2" /> Buy Asset
                            </Button>
                            <Button
                                className="bg-red-500 hover:bg-red-600 text-white border-2 border-black shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] hover:shadow-[2px_2px_0px_0px_rgba(0,0,0,1)] hover:translate-y-[2px] transition-all font-bold"
                            >
                                <Minus className="w-5 h-5 mr-2" /> Sell Asset
                            </Button>
                        </div>
                        <div className="h-8 w-0.5 bg-gray-300 hidden sm:block"></div>
                        <div className="flex items-center gap-4">
                            <div className="bg-white px-4 py-2 rounded border-2 border-black shadow-[2px_2px_0px_0px_rgba(0,0,0,1)]">
                                <Text className="text-xs text-gray-500 font-bold uppercase">Last Updated</Text>
                                <Text className="text-gray-900 font-mono font-bold">{new Date().toLocaleTimeString()}</Text>
                            </div>
                            <Button variant="default" size="icon" onClick={() => window.location.reload()}>
                                <RefreshCw className="w-5 h-5" />
                            </Button>
                        </div>
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