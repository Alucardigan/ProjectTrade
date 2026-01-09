import { Text } from "@/components/retroui/Text";
import { Button } from "@/components/retroui/Button";
import { ArrowLeft, Loader2 } from "lucide-react";
import { useNavigate } from "react-router-dom";
import { DashboardNavbar } from "@/components/CustomComponents/DashboardNavbar";
import { useQuery } from "@tanstack/react-query";
import { fetchOrderHistory } from "../api/orderManagement";
import { fetchTransactionHistory } from "../api/account";
import { PendingOrdersList } from "@/components/CustomComponents/PendingOrdersList";
import { TransactionHistoryTable } from "@/components/CustomComponents/TransactionHistoryTable";

const OrderHistoryPage = () => {
    const navigate = useNavigate();

    const { data: pendingOrders, isLoading: isLoadingOrders } = useQuery({
        queryKey: ['pendingOrders'],
        queryFn: fetchOrderHistory,
    });

    const { data: transactionHistory, isLoading: isLoadingTransactions } = useQuery({
        queryKey: ['transactionHistory'],
        queryFn: fetchTransactionHistory,
    });

    if (isLoadingOrders || isLoadingTransactions) {
        return (
            <div className="min-h-screen bg-yellow-50/50 font-sans flex items-center justify-center">
                <div className="flex flex-col items-center gap-4">
                    <Loader2 className="w-12 h-12 animate-spin text-black" />
                    <Text className="text-xl font-bold">Loading History...</Text>
                </div>
            </div>
        );
    }

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
                <PendingOrdersList orders={pendingOrders || []} />

                {/* Executed Orders Section */}
                <TransactionHistoryTable transactions={transactionHistory || []} />

            </div>
        </div>
    );
};

export default OrderHistoryPage;
