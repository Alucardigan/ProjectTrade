import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { Text } from "@/components/retroui/Text";
import { Button } from "@/components/retroui/Button";
import { Card } from "@/components/retroui/Card";
import { DashboardNavbar } from "@/components/CustomComponents/DashboardNavbar";
import { requestLoan, LoanType } from "../api/loans";
import { ArrowLeft, Banknote, Percent, ShieldAlert, Loader2 } from "lucide-react";

const LoanPage = () => {
    const navigate = useNavigate();
    const queryClient = useQueryClient();
    const [error, setError] = useState<string | null>(null);

    const { mutate: takeLoan, isPending } = useMutation({
        mutationFn: requestLoan,
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: ['accountBalance'] });
            navigate('/portfolio');
        },
        onError: (err: any) => {
            setError(err.response?.data?.error || "Failed to request loan. You may already have an active loan.");
        },
    });

    const handleLoanRequest = (type: LoanType) => {
        setError(null);
        takeLoan(type);
    };

    return (
        <div className="min-h-screen bg-yellow-50/50 font-sans pb-12">
            <DashboardNavbar />
            <div className="max-w-5xl mx-auto p-6 md:p-12">

                {/* Header */}
                <div className="mb-8">
                    <Button
                        variant="ghost"
                        onClick={() => navigate('/portfolio')}
                        className="mb-4 text-gray-600 hover:text-black pl-0"
                    >
                        <ArrowLeft className="w-4 h-4 mr-2" /> Back to Portfolio
                    </Button>
                    <div className="flex items-center gap-3 mb-2">
                        <div className="bg-green-100 p-2 rounded-lg border-2 border-black shadow-[2px_2px_0px_0px_rgba(0,0,0,1)]">
                            <Banknote className="w-8 h-8 text-green-700" />
                        </div>
                        <Text as="h1" className="text-4xl font-black text-gray-900 tracking-tight">Request a Loan</Text>
                    </div>
                    <Text className="text-gray-600 font-medium max-w-2xl">
                        Need more capital? Choose a loan plan that suits your trading strategy.
                        Warning: Failure to maintain solvency will result in bankruptcy and liquidation of assets.
                    </Text>
                </div>

                {error && (
                    <div className="mb-8 bg-red-100 border-2 border-black p-4 rounded-lg flex items-center gap-3 shadow-[4px_4px_0px_0px_rgba(0,0,0,1)]">
                        <ShieldAlert className="w-6 h-6 text-red-600" />
                        <Text className="font-bold text-red-800">{error}</Text>
                    </div>
                )}

                <div className="grid md:grid-cols-2 gap-8">
                    {/* Standard Loan */}
                    <Card className="relative overflow-hidden hover:translate-y-[-4px] transition-transform duration-200">
                        <div className="absolute top-0 right-0 bg-blue-100 px-4 py-2 border-b-2 border-l-2 border-black rounded-bl-lg">
                            <Text className="font-black text-blue-800">POPULAR</Text>
                        </div>

                        <div className="mb-6">
                            <Text as="h3" className="text-2xl font-black mb-2">Standard Loan</Text>
                            <Text className="text-gray-600">Perfect for getting started with leverage.</Text>
                        </div>

                        <div className="space-y-4 mb-8">
                            <div className="flex items-center justify-between p-3 bg-gray-50 rounded border-2 border-gray-200">
                                <div className="flex items-center gap-2">
                                    <Banknote className="w-5 h-5 text-gray-500" />
                                    <Text className="font-bold text-gray-700">Principal</Text>
                                </div>
                                <Text className="font-black text-2xl">$100,000</Text>
                            </div>
                            <div className="flex items-center justify-between p-3 bg-gray-50 rounded border-2 border-gray-200">
                                <div className="flex items-center gap-2">
                                    <Percent className="w-5 h-5 text-gray-500" />
                                    <Text className="font-bold text-gray-700">Interest Rate</Text>
                                </div>
                                <Text className="font-black text-xl text-green-600">5% p.a.</Text>
                            </div>
                        </div>

                        <Button
                            className="w-full py-6 text-lg"
                            onClick={() => handleLoanRequest(LoanType.Standard)}
                            disabled={isPending}
                        >
                            {isPending ? <Loader2 className="animate-spin mr-2" /> : null}
                            Take Standard Loan
                        </Button>
                    </Card>

                    {/* Premium Loan */}
                    <Card className="relative overflow-hidden bg-gray-900 text-white hover:translate-y-[-4px] transition-transform duration-200">
                        <div className="absolute top-0 right-0 bg-yellow-400 px-4 py-2 border-b-2 border-l-2 border-white rounded-bl-lg">
                            <Text className="font-black text-black">PRO TRADER</Text>
                        </div>

                        <div className="mb-6">
                            <Text as="h3" className="text-2xl font-black mb-2 text-white">Premium Loan</Text>
                            <Text className="text-gray-400">High capital for aggressive strategies.</Text>
                        </div>

                        <div className="space-y-4 mb-8">
                            <div className="flex items-center justify-between p-3 bg-gray-800 rounded border-2 border-gray-700">
                                <div className="flex items-center gap-2">
                                    <Banknote className="w-5 h-5 text-gray-400" />
                                    <Text className="font-bold text-gray-300">Principal</Text>
                                </div>
                                <Text className="font-black text-2xl text-yellow-400">$1,000,000</Text>
                            </div>
                            <div className="flex items-center justify-between p-3 bg-gray-800 rounded border-2 border-gray-700">
                                <div className="flex items-center gap-2">
                                    <Percent className="w-5 h-5 text-gray-400" />
                                    <Text className="font-bold text-gray-300">Interest Rate</Text>
                                </div>
                                <Text className="font-black text-xl text-red-400">10% p.a.</Text>
                            </div>
                        </div>

                        <Button
                            variant="secondary"
                            className="w-full py-6 text-lg bg-yellow-400 hover:bg-yellow-500 text-black border-white"
                            onClick={() => handleLoanRequest(LoanType.Premium)}
                            disabled={isPending}
                        >
                            {isPending ? <Loader2 className="animate-spin mr-2" /> : null}
                            Take Premium Loan
                        </Button>
                    </Card>
                </div>

                <div className="mt-12 bg-blue-50 border-2 border-blue-200 p-6 rounded-lg">
                    <div className="flex items-start gap-4">
                        <div className="bg-blue-100 p-2 rounded-full border-2 border-blue-200">
                            <ShieldAlert className="w-6 h-6 text-blue-600" />
                        </div>
                        <div>
                            <Text className="font-black text-lg text-blue-900 mb-2">Understanding Bankruptcy Risk</Text>
                            <Text className="text-blue-800 leading-relaxed">
                                Taking a loan increases your buying power but also your risk. If your portfolio value drops significantly,
                                your equity percentage may fall below the maintenance requirement (25%). In this event, your assets
                                will be liquidated to repay the loan, and you will be declared bankrupt. Trade wisely.
                            </Text>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
};

export default LoanPage;
