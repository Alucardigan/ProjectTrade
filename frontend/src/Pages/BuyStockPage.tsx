import { useState } from "react";
import { DashboardNavbar } from "@/components/CustomComponents/DashboardNavbar";
import { Card } from "@/components/retroui/Card";
import { Text } from "@/components/retroui/Text";
import { Button } from "@/components/retroui/Button";
import { Badge } from "@/components/retroui/Badge";
import { Search, ArrowRight, TrendingUp, AlertCircle } from "lucide-react";
import { useNavigate } from "react-router-dom";
import { useMutation } from "@tanstack/react-query";
import { placeOrder } from "../api/orderManagement";
import { OrderType } from "../types/OrderType";

const BuyStockPage = () => {
    const navigate = useNavigate();
    const [ticker, setTicker] = useState("");
    const [quantity, setQuantity] = useState("");
    const [estimatedPrice, setEstimatedPrice] = useState<number | null>(null);

    const mutation = useMutation({
        mutationFn: placeOrder,
        onSuccess: () => {
            navigate('/portfolio');
        },
        onError: (error) => {
            console.error("Failed to place order:", error);
            alert("Failed to place order. Please try again.");
        }
    });

    const recommendations = [
        { symbol: "NVDA", name: "NVIDIA Corp", price: 485.09, change: "+2.5%", isPositive: true },
        { symbol: "MSFT", name: "Microsoft", price: 375.25, change: "+1.2%", isPositive: true },
        { symbol: "GOOGL", name: "Alphabet Inc", price: 138.50, change: "-0.5%", isPositive: false },
        { symbol: "AMZN", name: "Amazon.com", price: 145.20, change: "+0.8%", isPositive: true },
    ];

    const updateTicker = (value: string) => {
        setTicker(value);
        // Mock price fetch logic
        if (value.length >= 3) {
            // Check if it's one of our recommendations to use the "real" mock price
            const rec = recommendations.find(r => r.symbol === value);
            if (rec) {
                setEstimatedPrice(rec.price);
            } else {
                // Random price between 10 and 500
                const mockPrice = Math.floor(Math.random() * 490) + 10;
                setEstimatedPrice(mockPrice);
            }
        } else {
            setEstimatedPrice(null);
        }
    };

    const handleTickerChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        updateTicker(e.target.value.toUpperCase());
    };

    const totalCost = estimatedPrice && quantity ? estimatedPrice * Number(quantity) : 0;

    const handleBuy = () => {
        if (!ticker || !quantity) return;

        mutation.mutate({
            symbol: ticker,
            quantity: Number(quantity),
            order_type: OrderType.Buy,
            price_buffer: 0
        });
    };

    return (
        <div className="min-h-screen bg-yellow-50/50 font-sans pb-12">
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

                <Card className="bg-white border-4 border-black shadow-[8px_8px_0px_0px_rgba(0,0,0,1)] mb-12">
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
                            disabled={!ticker || !quantity || mutation.isPending}
                            className="w-full py-6 text-xl bg-green-500 hover:bg-green-600 text-white border-2 border-black shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] hover:shadow-[2px_2px_0px_0px_rgba(0,0,0,1)] hover:translate-y-[2px] transition-all disabled:opacity-50 disabled:cursor-not-allowed disabled:transform-none disabled:shadow-none"
                        >
                            {mutation.isPending ? "Processing..." : (
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

                {/* Recommendations Section */}
                <div>
                    <Text as="h2" className="text-2xl font-black text-gray-900 mb-6 flex items-center gap-2">
                        <TrendingUp className="w-6 h-6" /> Trending Assets
                    </Text>
                    <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                        {recommendations.map((rec) => (
                            <Card
                                key={rec.symbol}
                                className="bg-white border-2 border-black shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] hover:translate-y-[2px] hover:shadow-[2px_2px_0px_0px_rgba(0,0,0,1)] transition-all cursor-pointer group"
                                onClick={() => updateTicker(rec.symbol)}
                            >
                                <Card.Content className="p-5 flex justify-between items-center">
                                    <div>
                                        <div className="flex items-center gap-2 mb-1">
                                            <Text className="font-black text-xl group-hover:text-blue-600 transition-colors">{rec.symbol}</Text>
                                            <Badge variant={rec.isPositive ? "success" : "destructive"} className="text-[10px] px-1.5 py-0 h-5 border border-black">
                                                {rec.change}
                                            </Badge>
                                        </div>
                                        <Text className="text-gray-500 font-medium text-xs uppercase tracking-wide">{rec.name}</Text>
                                    </div>
                                    <div className="text-right">
                                        <Text className="font-bold text-lg">${rec.price.toFixed(2)}</Text>
                                        <div className="text-xs font-bold text-blue-600 uppercase tracking-wider mt-1 opacity-0 group-hover:opacity-100 transition-opacity">Select</div>
                                    </div>
                                </Card.Content>
                            </Card>
                        ))}
                    </div>
                </div>
            </div>
        </div>
    );
};

export default BuyStockPage;
