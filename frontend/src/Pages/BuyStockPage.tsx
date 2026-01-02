import { useState } from "react";
import { DashboardNavbar } from "@/components/CustomComponents/DashboardNavbar";
import { Card } from "@/components/retroui/Card";
import { Text } from "@/components/retroui/Text";
import { Button } from "@/components/retroui/Button";
import { Badge } from "@/components/retroui/Badge";
import { Search, DollarSign, ArrowRight, TrendingUp, AlertCircle } from "lucide-react";
import { useNavigate } from "react-router-dom";

const BuyStockPage = () => {
    const navigate = useNavigate();
    const [ticker, setTicker] = useState("");
    const [quantity, setQuantity] = useState("");
    const [estimatedPrice, setEstimatedPrice] = useState<number | null>(null);
    const [isLoading, setIsLoading] = useState(false);

    // Mock function to simulate fetching price
    const handleTickerChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const value = e.target.value.toUpperCase();
        setTicker(value);

        // Mock price fetch logic
        if (value.length >= 3) {
            // Random price between 10 and 500
            const mockPrice = Math.floor(Math.random() * 490) + 10;
            setEstimatedPrice(mockPrice);
        } else {
            setEstimatedPrice(null);
        }
    };

    const totalCost = estimatedPrice && quantity ? estimatedPrice * Number(quantity) : 0;

    const handleBuy = () => {
        setIsLoading(true);
        // Simulate API call
        setTimeout(() => {
            setIsLoading(false);
            navigate('/portfolio');
        }, 1500);
    };

    return (
        <div className="min-h-screen bg-yellow-50/50 font-sans">
            <DashboardNavbar />
            <div className="max-w-2xl mx-auto p-6 md:p-12">
                <div className="mb-8">
                    <Button
                        variant="ghost"
                        onClick={() => navigate('/portfolio')}
                        className="mb-4 text-gray-600 hover:text-black pl-0"
                    >
                        ← Back to Portfolio
                    </Button>
                    <Text as="h1" className="text-4xl font-black text-gray-900 tracking-tight">Buy Asset</Text>
                    <Text className="text-gray-600 font-medium mt-2">Search for a stock and add it to your portfolio.</Text>
                </div>

                <Card className="bg-white border-4 border-black shadow-[8px_8px_0px_0px_rgba(0,0,0,1)]">
                    <Card.Content className="p-8 space-y-8">

                        {/* Ticker Input */}
                        <div className="space-y-2">
                            <label className="text-sm font-bold uppercase tracking-wider text-gray-500">Stock Ticker</label>
                            <div className="relative">
                                <Search className="absolute left-4 top-1/2 transform -translate-y-1/2 text-gray-400 w-5 h-5" />
                                <input
                                    type="text"
                                    value={ticker}
                                    onChange={handleTickerChange}
                                    placeholder="e.g. AAPL, TSLA"
                                    className="w-full pl-12 pr-4 py-4 bg-gray-50 border-2 border-black rounded text-xl font-bold focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all placeholder:text-gray-300 uppercase"
                                />
                            </div>
                            {estimatedPrice && (
                                <div className="flex items-center gap-2 mt-2 animate-in fade-in slide-in-from-top-2">
                                    <Badge variant="success" className="border-2 border-black">
                                        <TrendingUp className="w-3 h-3 mr-1" /> Live Price
                                    </Badge>
                                    <Text className="font-mono font-bold text-lg">${estimatedPrice.toFixed(2)}</Text>
                                </div>
                            )}
                        </div>

                        {/* Quantity Input */}
                        <div className="space-y-2">
                            <label className="text-sm font-bold uppercase tracking-wider text-gray-500">Quantity</label>
                            <div className="relative">
                                <input
                                    type="number"
                                    value={quantity}
                                    onChange={(e) => setQuantity(e.target.value)}
                                    placeholder="0"
                                    min="1"
                                    className="w-full px-4 py-4 bg-gray-50 border-2 border-black rounded text-xl font-bold focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all placeholder:text-gray-300"
                                />
                            </div>
                        </div>

                        {/* Order Summary */}
                        <div className="bg-blue-50 p-6 rounded border-2 border-black border-dashed">
                            <div className="flex justify-between items-center mb-2">
                                <Text className="text-gray-600 font-medium">Estimated Price</Text>
                                <Text className="font-bold">${estimatedPrice ? estimatedPrice.toFixed(2) : "0.00"}</Text>
                            </div>
                            <div className="flex justify-between items-center mb-4">
                                <Text className="text-gray-600 font-medium">Quantity</Text>
                                <Text className="font-bold">{quantity || "0"}</Text>
                            </div>
                            <div className="border-t-2 border-black border-dashed my-4"></div>
                            <div className="flex justify-between items-end">
                                <Text className="text-lg font-bold uppercase">Total Cost</Text>
                                <Text className="text-3xl font-black text-blue-600">
                                    ${totalCost.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })}
                                </Text>
                            </div>
                        </div>

                        {/* Action Button */}
                        <Button
                            onClick={handleBuy}
                            disabled={!ticker || !quantity || isLoading}
                            className="w-full py-6 text-xl bg-green-500 hover:bg-green-600 text-white border-2 border-black shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] hover:shadow-[2px_2px_0px_0px_rgba(0,0,0,1)] hover:translate-y-[2px] transition-all disabled:opacity-50 disabled:cursor-not-allowed disabled:transform-none disabled:shadow-none"
                        >
                            {isLoading ? "Processing..." : (
                                <span className="flex items-center justify-center gap-2">
                                    Confirm Purchase <ArrowRight className="w-6 h-6" />
                                </span>
                            )}
                        </Button>

                        <div className="flex items-center justify-center gap-2 text-gray-500 text-sm font-medium">
                            <AlertCircle className="w-4 h-4" />
                            Market orders are executed immediately at the best available price.
                        </div>

                    </Card.Content>
                </Card>
            </div>
        </div>
    );
};

export default BuyStockPage;
