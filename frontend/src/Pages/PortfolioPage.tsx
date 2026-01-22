import { Text } from "@/components/retroui/Text";
import { PortfolioSummary } from "@/components/CustomComponents/PortfolioSummary";
import { HoldingsGrid } from "@/components/CustomComponents/HoldingsGrid";
import { useMemo } from "react";
import { Button } from "@/components/retroui/Button";
import { RefreshCw, Plus, Loader2 } from "lucide-react";
import { DashboardNavbar } from "@/components/CustomComponents/DashboardNavbar";
import { useQuery } from "@tanstack/react-query";
import { fetchPortfolio } from "../api/portfolio";
import { getLoan } from "../api/loans";
import type { PortfolioResponse } from "../types/Portfolio_Response";
import { useNavigate } from "react-router-dom";

const defaultPortfolioResponse: PortfolioResponse = {
    user_id: "guest",
    portfolio: []
};

const PortfolioPage = () => {
    const navigate = useNavigate();
    const { data: portfolioResponse, isLoading: isPortfolioLoading, refetch: refetchPortfolio } = useQuery({
        queryKey: ['portfolio'],
        queryFn: fetchPortfolio,
    });

    const { data: loan, isLoading: isLoanLoading, refetch: refetchLoan } = useQuery({
        queryKey: ['loan'],
        queryFn: getLoan,
        retry: false,
    });

    const activePortfolio = portfolioResponse || defaultPortfolioResponse;

    const totalValue = useMemo(() => {
        return activePortfolio.portfolio.reduce((acc, item) =>
            acc + Number(item.total_money_spent) + Number(item.total_profit), 0);
    }, [activePortfolio]);

    const totalGain = useMemo(() => {
        return activePortfolio.portfolio.reduce((acc, item) =>
            acc + Number(item.total_profit), 0);
    }, [activePortfolio]);

    const totalCost = useMemo(() => {
        return activePortfolio.portfolio.reduce((acc, item) =>
            acc + Number(item.total_money_spent), 0);
    }, [activePortfolio]);

    const gainPercentage = totalCost > 0 ? (totalGain / totalCost) * 100 : 0;

    const totalLiabilities = useMemo(() => {
        if (!loan) return 0;
        // If the API returns an error structure or empty object, handle it
        if (!loan.principal) return 0;

        const principal = Number(loan.principal);
        const rate = Number(loan.interest_rate);
        const lastPaid = new Date(loan.last_paid_at);
        const now = new Date();

        // Calculate days elapsed
        const timeDiff = now.getTime() - lastPaid.getTime();
        const days = Math.floor(timeDiff / (1000 * 3600 * 24));

        // Backend formula: 
        // Daily rate = (rate / 100) / 365
        // Balance = Principal * (1 + Daily Rate) ^ Days

        const dailyRate = (rate / 100) / 365;
        const interestRate = 1 + dailyRate;
        const interestRateOverTime = Math.pow(interestRate, Math.max(0, days));

        return principal * interestRateOverTime;
    }, [loan]);

    const handleRefetch = () => {
        refetchPortfolio();
        refetchLoan();
    };

    // Only show loading if portfolio is loading. 
    // Loan loading is secondary, or if it fails (404) we just show 0.
    if (isPortfolioLoading) {
        return (
            <div className="min-h-screen bg-yellow-50/50 font-sans flex items-center justify-center">
                <div className="flex flex-col items-center gap-4">
                    <Loader2 className="w-12 h-12 animate-spin text-black" />
                    <Text className="text-xl font-bold">Loading Portfolio...</Text>
                </div>
            </div>
        );
    }

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
                                onClick={() => navigate('/buy')}

                            >
                                <Plus className="w-5 h-5 mr-2" /> Buy Asset
                            </Button>

                        </div>
                        <div className="h-8 w-0.5 bg-gray-300 hidden sm:block"></div>
                        <div className="flex items-center gap-4">
                            <div className="bg-white px-4 py-2 rounded border-2 border-black shadow-[2px_2px_0px_0px_rgba(0,0,0,1)]">
                                <Text className="text-xs text-gray-500 font-bold uppercase">Last Updated</Text>
                                <Text className="text-gray-900 font-mono font-bold">{new Date().toLocaleTimeString()}</Text>
                            </div>
                            <Button variant="default" size="icon" onClick={handleRefetch}>
                                <RefreshCw className={`w-5 h-5 ${isLoanLoading ? 'animate-spin' : ''}`} />
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
                    totalLiabilities={totalLiabilities}
                />

                {/* Holdings List */}
                <HoldingsGrid portfolio={activePortfolio.portfolio} />
            </div>
        </div>
    );
}

export default PortfolioPage;