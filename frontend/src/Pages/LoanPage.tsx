import { useState, useMemo } from 'react';
import { useNavigate } from 'react-router-dom';
import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query';
import { Text } from "@/components/retroui/Text";
import { Button } from "@/components/retroui/Button";
import { Card } from "@/components/retroui/Card";
import { DashboardNavbar } from "@/components/CustomComponents/DashboardNavbar";
import { requestLoan, getLoan, repayLoan, LoanType } from "../api/loans";
import { ArrowLeft, Banknote, Percent, ShieldAlert, Loader2, CheckCircle2 } from "lucide-react";

const LoanPage = () => {
    const navigate = useNavigate();
    const queryClient = useQueryClient();
    const [error, setError] = useState<string | null>(null);
    const [repayAmount, setRepayAmount] = useState<string>('');

    // Fetch current loan status
    const { data: activeLoan, isLoading: isLoadingLoan } = useQuery({
        queryKey: ['loan'],
        queryFn: getLoan,
        retry: false,
    });

    // Mutation to take a loan
    const { mutate: takeLoan, isPending: isTakingLoan } = useMutation({
        mutationFn: requestLoan,
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: ['loan'] });
            queryClient.invalidateQueries({ queryKey: ['accountBalance'] });
            setError(null);
        },
        onError: (err: any) => {
            setError(err.response?.data?.error || "Failed to request loan.");
        },
    });

    // Mutation to repay loan
    const { mutate: payLoan, isPending: isRepaying } = useMutation({
        mutationFn: repayLoan,
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: ['loan'] });
            queryClient.invalidateQueries({ queryKey: ['accountBalance'] });
            setRepayAmount('');
            setError(null);
        },
        onError: (err: any) => {
            setError(err.response?.data?.error || "Failed to repay loan.");
        },
    });

    const handleLoanRequest = (type: LoanType) => {
        setError(null);
        takeLoan(type);
    };

    const handleRepayment = () => {
        if (!repayAmount || isNaN(Number(repayAmount)) || Number(repayAmount) <= 0) {
            setError("Please enter a valid repayment amount.");
            return;
        }
        payLoan(Number(repayAmount));
    };

    // Calculate dynamic loan details
    const loanDetails = useMemo(() => {
        if (!activeLoan || !activeLoan.principal) return null;

        const principal = Number(activeLoan.principal);
        const rate = Number(activeLoan.interest_rate);
        const lastPaid = new Date(activeLoan.last_paid_at);
        const now = new Date();

        const timeDiff = now.getTime() - lastPaid.getTime();
        const days = Math.floor(timeDiff / (1000 * 3600 * 24));

        const dailyRate = (rate / 100) / 365;
        const interestRate = 1 + dailyRate;
        const interestRateOverTime = Math.pow(interestRate, Math.max(0, days));

        const totalDue = principal * interestRateOverTime;
        const accruedInterest = totalDue - principal;

        return {
            principal,
            accruedInterest,
            totalDue
        };
    }, [activeLoan]);

    if (isLoadingLoan) {
        return (
            <div className="min-h-screen bg-yellow-50/50 font-sans flex items-center justify-center">
                <Loader2 className="w-12 h-12 animate-spin text-black" />
            </div>
        );
    }

    const hasActiveLoan = activeLoan && activeLoan.status === 'ONGOING';

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
                        <Text as="h1" className="text-4xl font-black text-gray-900 tracking-tight">
                            {hasActiveLoan ? "Manage Loan" : "Request a Loan"}
                        </Text>
                    </div>
                </div>

                {error && (
                    <div className="mb-8 bg-red-100 border-2 border-black p-4 rounded-lg flex items-center gap-3 shadow-[4px_4px_0px_0px_rgba(0,0,0,1)]">
                        <ShieldAlert className="w-6 h-6 text-red-600" />
                        <Text className="font-bold text-red-800">{error}</Text>
                    </div>
                )}

                {hasActiveLoan && loanDetails ? (
                    /* REPAYMENT DASHBOARD */
                    <div className="space-y-8">
                        <Card className="bg-white border-2 border-black shadow-[4px_4px_0px_0px_rgba(0,0,0,1)]">
                            <Card.Content className="p-8">
                                <div className="flex flex-col md:flex-row justify-between items-start md:items-center mb-8 gap-4">
                                    <div>
                                        <Text className="text-gray-500 font-bold uppercase tracking-wider text-sm">Total Outstanding Balance</Text>
                                        <Text as="h2" className="text-5xl font-black text-gray-900 mt-2">
                                            ${loanDetails.totalDue.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })}
                                        </Text>
                                        <div className="flex gap-4 mt-2 text-sm font-medium text-gray-600">
                                            <span>Principal: ${loanDetails.principal.toLocaleString(undefined, { minimumFractionDigits: 2 })}</span>
                                            <span className="text-red-600">+ Interest: ${loanDetails.accruedInterest.toLocaleString(undefined, { minimumFractionDigits: 2 })}</span>
                                        </div>
                                    </div>
                                    <div className="bg-blue-50 px-4 py-2 rounded border-2 border-blue-200">
                                        <Text className="text-blue-800 font-bold">Interest Rate: {Number(activeLoan.interest_rate)}% p.a.</Text>
                                    </div>
                                </div>

                                {/* Progress Bar */}
                                <div className="mb-8">
                                    <div className="flex justify-between text-sm font-bold mb-2">
                                        <Text>Repayment Progress</Text>
                                        <Text>0% Paid</Text>
                                    </div>
                                    <div className="h-4 bg-gray-200 rounded-full border-2 border-black overflow-hidden">
                                        {/* Since we don't track original loan amount easily here without extra logic, we'll just show a full bar for outstanding. 
                                            Ideally, you'd want (Original - Current) / Original. For now, let's just show a visual indicator of debt. */}
                                        <div className="h-full bg-red-500 w-full animate-pulse" title="Outstanding Debt"></div>
                                    </div>
                                    <Text className="text-xs text-gray-500 mt-2 text-right">Debt is accruing interest daily.</Text>
                                </div>

                                {/* Repayment Form */}
                                <div className="bg-gray-50 p-6 rounded-lg border-2 border-gray-200">
                                    <Text className="font-bold text-lg mb-4">Make a Payment</Text>
                                    <div className="flex flex-col md:flex-row gap-4">
                                        <div className="flex-1 relative">
                                            <div className="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                                                <span className="text-gray-500 font-bold">$</span>
                                            </div>
                                            <input
                                                type="number"
                                                value={repayAmount}
                                                onChange={(e) => setRepayAmount(e.target.value)}
                                                placeholder="Enter amount"
                                                className="w-full pl-8 pr-4 py-3 bg-white border-2 border-black rounded font-bold focus:outline-none focus:ring-2 focus:ring-black focus:border-transparent transition-all"
                                            />
                                        </div>
                                        <Button
                                            onClick={handleRepayment}
                                            disabled={isRepaying}
                                            className="md:w-48 py-3"
                                        >
                                            {isRepaying ? <Loader2 className="animate-spin mr-2" /> : <CheckCircle2 className="w-5 h-5 mr-2" />}
                                            Repay Loan
                                        </Button>
                                    </div>
                                    <div className="flex gap-2 mt-4">
                                        <Button variant="outline" size="sm" onClick={() => setRepayAmount(loanDetails.accruedInterest.toFixed(2))}>
                                            Pay Interest Only
                                        </Button>
                                        <Button variant="outline" size="sm" onClick={() => setRepayAmount(loanDetails.totalDue.toFixed(2))}>
                                            Pay Full Amount
                                        </Button>
                                    </div>
                                </div>
                            </Card.Content>
                        </Card>
                    </div>
                ) : (
                    /* LOAN OPTIONS (If no active loan) */
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
                                disabled={isTakingLoan}
                            >
                                {isTakingLoan ? <Loader2 className="animate-spin mr-2" /> : null}
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
                                disabled={isTakingLoan}
                            >
                                {isTakingLoan ? <Loader2 className="animate-spin mr-2" /> : null}
                                Take Premium Loan
                            </Button>
                        </Card>
                    </div>
                )}

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
