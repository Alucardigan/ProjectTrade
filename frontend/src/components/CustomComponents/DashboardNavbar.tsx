import { useNavigate } from 'react-router-dom';
import { Button } from "@/components/retroui/Button";
import { Text } from "@/components/retroui/Text";
import { BarChart3, LogOut, Wallet, Loader2, Clock, Banknote } from "lucide-react";
import { useQuery } from "@tanstack/react-query";
import { fetchAccountBalance } from "../../api/account";

export const DashboardNavbar = () => {
    const navigate = useNavigate();

    const { data: accountData, isLoading } = useQuery({
        queryKey: ['accountBalance'],
        queryFn: fetchAccountBalance,
    });

    return (
        <nav className="border-b-4 border-black bg-white px-6 py-4 sticky top-0 z-50">
            <div className="max-w-7xl mx-auto flex justify-between items-center">
                <div className="flex items-center gap-2 cursor-pointer" onClick={() => navigate('/landing_page')}>
                    <div className="bg-black text-white p-2 rounded-lg transform -rotate-3">
                        <BarChart3 className="w-6 h-6" />
                    </div>
                    <Text as="h3" className="text-2xl font-black tracking-tighter">RETRO<span className="text-blue-600">TRADE</span></Text>
                </div>
                <div className="flex items-center gap-6">
                    {/* Balance Display */}
                    <div className="hidden md:flex items-center gap-2 bg-green-100 px-4 py-2 rounded-full border-2 border-black shadow-[2px_2px_0px_0px_rgba(0,0,0,1)]">
                        <Wallet className="w-4 h-4 text-green-700" />
                        <div className="flex flex-col leading-none">
                            <span className="text-[10px] font-bold uppercase text-green-800 tracking-wider">Available</span>
                            {isLoading ? (
                                <Loader2 className="w-4 h-4 animate-spin mt-1" />
                            ) : (
                                <span className="font-black text-green-900">
                                    ${Number(accountData?.available_balance ?? 0).toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })}
                                </span>
                            )}
                        </div>
                    </div>

                    <Button
                        variant="ghost"
                        onClick={() => navigate('/loans')}
                        className="font-bold flex items-center gap-2"
                    >
                        <Banknote className="w-4 h-4" /> Loans
                    </Button>

                    <Button
                        variant="ghost"
                        onClick={() => navigate('/orders')}
                        className="font-bold flex items-center gap-2"
                    >
                        <Clock className="w-4 h-4" /> History
                    </Button>

                    <Button
                        variant="ghost"
                        onClick={() => navigate('/landing_page')}
                        className="font-bold flex items-center gap-2"
                    >
                        <LogOut className="w-4 h-4" /> Logout
                    </Button>
                </div>
            </div>
        </nav>
    );
};
