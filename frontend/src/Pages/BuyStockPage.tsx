import { useState, useEffect } from "react";
import { DashboardNavbar } from "@/components/CustomComponents/DashboardNavbar";
import { Card } from "@/components/retroui/Card";
import { Text } from "@/components/retroui/Text";
import { Button } from "@/components/retroui/Button";
import { Badge } from "@/components/retroui/Badge";
import { Search, ArrowRight, TrendingUp, TrendingDown, AlertCircle, BarChart2, Zap } from "lucide-react";
import { useNavigate, useSearchParams } from "react-router-dom";
import { useMutation, useQuery } from "@tanstack/react-query";
import { placeOrder } from "../api/orderManagement";
import { fetchAllTickers } from "../api/ticker";
import { OrderType } from "../types/OrderType";
import type { TickerSummary } from "../types/TickerSummary";

const BuyStockPage = () => {
    const navigate = useNavigate();
    const [searchParams] = useSearchParams();
    const initialSymbol = searchParams.get("symbol") || "";

    const [ticker, setTicker] = useState(initialSymbol);
    const [searchQuery, setSearchQuery] = useState("");
    const [quantity, setQuantity] = useState("");
    const [selectedCompany, setSelectedCompany] = useState<TickerSummary | null>(null);

    const { data: tickers = [], isLoading } = useQuery<TickerSummary[]>({
        queryKey: ['tickers'],
        queryFn: fetchAllTickers,
        refetchInterval: 5000,
    });

    useEffect(() => {
        if (initialSymbol && tickers.length > 0) {
            const found = tickers.find(t => t.symbol.toUpperCase() === initialSymbol.toUpperCase());
            if (found) {
                setSelectedCompany(found);
                setTicker(found.symbol);
            }
        }
    }, [initialSymbol, tickers]);

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

    const handleSelectTicker = (item: TickerSummary) => {
        setTicker(item.symbol);
        setSelectedCompany(item);
    };

    const handleTickerInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const val = e.target.value.toUpperCase();
        setTicker(val);
        const match = tickers.find(t => t.symbol === val);
        if (match) {
            setSelectedCompany(match);
        } else {
            setSelectedCompany(null);
        }
    };

    const estimatedPrice = selectedCompany ? Number(selectedCompany.current_price) : null;
    const totalCost = estimatedPrice && quantity ? estimatedPrice * Number(quantity) : 0;

    const handleBuy = () => {
        if (!ticker || !quantity) return;

        mutation.mutate({
            ticker: ticker,
            quantity: Number(quantity),
            order_type: OrderType.Buy,
            price_buffer: 0
        });
    };

    const filteredTickers = tickers.filter(t =>
        t.symbol.toLowerCase().includes(searchQuery.toLowerCase()) ||
        t.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
        t.sector.toLowerCase().includes(searchQuery.toLowerCase())
    );

    return (
        <div className="min-h-screen bg-yellow-50/50 font-sans pb-12">
            <DashboardNavbar />
            <div className="max-w-4xl mx-auto p-6 md:p-12">
                <div className="mb-8">
                    <Button
                        variant="ghost"
                        onClick={() => navigate('/portfolio')}
                        className="mb-4 text-gray-600 hover:text-black pl-0"
                    >
                        ← Back to Portfolio
                    </Button>
                    <div className="flex items-center gap-3">
                        <Text as="h1" className="text-4xl font-black text-gray-900 tracking-tight">Trade Synthetic Assets</Text>
                        <Badge variant="retro" className="border-2 border-black bg-blue-100 text-blue-900 font-bold">
                            <Zap className="w-3.5 h-3.5 mr-1 text-blue-600 inline" /> Live Simulation
                        </Badge>
                    </div>
                    <Text className="text-gray-600 font-medium mt-2">
                        Execute real-time orders across our synthetic stock universe with simulated order books.
                    </Text>
                </div>

                <div className="grid grid-cols-1 lg:grid-cols-12 gap-8 mb-12">
                    {/* Order Panel */}
                    <div className="lg:col-span-6">
                        <Card className="bg-white border-4 border-black shadow-[8px_8px_0px_0px_rgba(0,0,0,1)]">
                            <Card.Content className="p-8 space-y-6">
                                {/* Ticker Input */}
                                <div className="space-y-2">
                                    <label className="text-sm font-bold uppercase tracking-wider text-gray-500">Asset Symbol</label>
                                    <div className="relative">
                                        <Search className="absolute left-4 top-1/2 transform -translate-y-1/2 text-gray-400 w-5 h-5" />
                                        <input
                                            type="text"
                                            value={ticker}
                                            onChange={handleTickerInputChange}
                                            placeholder="e.g. CYBR, WAYN, ACME"
                                            className="w-full pl-12 pr-4 py-3 bg-gray-50 border-2 border-black rounded text-xl font-bold focus:outline-none focus:ring-2 focus:ring-blue-500 uppercase"
                                        />
                                    </div>
                                    {selectedCompany && (
                                        <div className="p-3 bg-gray-50 border-2 border-black rounded-md mt-2 flex justify-between items-center">
                                            <div>
                                                <Text className="font-bold text-sm text-gray-900">{selectedCompany.name}</Text>
                                                <Text className="text-xs text-gray-500">{selectedCompany.sector}</Text>
                                            </div>
                                            <div className="text-right">
                                                <div className="flex items-center gap-1">
                                                    {selectedCompany.is_positive ? (
                                                        <TrendingUp className="w-4 h-4 text-green-600" />
                                                    ) : (
                                                        <TrendingDown className="w-4 h-4 text-red-600" />
                                                    )}
                                                    <span className={`text-xs font-bold ${selectedCompany.is_positive ? 'text-green-600' : 'text-red-600'}`}>
                                                        {selectedCompany.is_positive ? '+' : ''}{selectedCompany.change_percent}%
                                                    </span>
                                                </div>
                                                <Button
                                                    size="sm"
                                                    variant="ghost"
                                                    onClick={() => navigate(`/stocks/${selectedCompany.symbol}`)}
                                                    className="text-xs text-blue-600 hover:text-blue-800 p-0 h-auto font-bold flex items-center gap-1"
                                                >
                                                    <BarChart2 className="w-3 h-3" /> View Chart
                                                </Button>
                                            </div>
                                        </div>
                                    )}
                                </div>

                                {/* Quantity Input */}
                                <div className="space-y-2">
                                    <label className="text-sm font-bold uppercase tracking-wider text-gray-500">Quantity (Shares)</label>
                                    <input
                                        type="number"
                                        value={quantity}
                                        onChange={(e) => setQuantity(e.target.value)}
                                        placeholder="0"
                                        min="1"
                                        className="w-full px-4 py-3 bg-gray-50 border-2 border-black rounded text-xl font-bold focus:outline-none focus:ring-2 focus:ring-blue-500"
                                    />
                                </div>

                                {/* Order Summary */}
                                <div className="bg-blue-50 p-5 rounded border-2 border-black border-dashed">
                                    <div className="flex justify-between items-center mb-2">
                                        <Text className="text-gray-600 font-medium">Fair Market Price</Text>
                                        <Text className="font-bold font-mono">
                                            {estimatedPrice ? `$${estimatedPrice.toFixed(2)}` : "—"}
                                        </Text>
                                    </div>
                                    <div className="flex justify-between items-center mb-2">
                                        <Text className="text-gray-600 font-medium">Quantity</Text>
                                        <Text className="font-bold">{quantity || "0"}</Text>
                                    </div>
                                    <div className="border-t-2 border-black border-dashed my-3"></div>
                                    <div className="flex justify-between items-end">
                                        <Text className="text-lg font-bold uppercase">Total Cost</Text>
                                        <Text className="text-2xl font-black text-blue-600 font-mono">
                                            ${totalCost.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })}
                                        </Text>
                                    </div>
                                </div>

                                {/* Action Button */}
                                <Button
                                    onClick={handleBuy}
                                    disabled={!ticker || !quantity || mutation.isPending || !estimatedPrice}
                                    className="w-full py-5 text-xl bg-green-500 hover:bg-green-600 text-white border-2 border-black shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] hover:shadow-[2px_2px_0px_0px_rgba(0,0,0,1)] hover:translate-y-[2px] transition-all disabled:opacity-50 disabled:cursor-not-allowed"
                                >
                                    {mutation.isPending ? "Executing Order..." : (
                                        <span className="flex items-center justify-center gap-2">
                                            Confirm Purchase <ArrowRight className="w-5 h-5" />
                                        </span>
                                    )}
                                </Button>

                                <div className="flex items-center justify-center gap-2 text-gray-500 text-xs font-medium text-center">
                                    <AlertCircle className="w-4 h-4 flex-shrink-0" />
                                    Orders match instantly against market maker liquidity quotes.
                                </div>
                            </Card.Content>
                        </Card>
                    </div>

                    {/* Stock Universe Catalog */}
                    <div className="lg:col-span-6 space-y-4">
                        <div className="flex justify-between items-center">
                            <Text as="h2" className="text-xl font-black text-gray-900 flex items-center gap-2">
                                Synthetic Companies Catalog
                            </Text>
                            <span className="text-xs font-bold text-gray-500 uppercase">{filteredTickers.length} Assets</span>
                        </div>

                        <div className="relative mb-3">
                            <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400 w-4 h-4" />
                            <input
                                type="text"
                                value={searchQuery}
                                onChange={(e) => setSearchQuery(e.target.value)}
                                placeholder="Filter by name, symbol or sector..."
                                className="w-full pl-9 pr-3 py-2 bg-white border-2 border-black rounded text-sm font-medium focus:outline-none focus:ring-2 focus:ring-blue-500"
                            />
                        </div>

                        {isLoading ? (
                            <div className="p-8 text-center text-gray-500 font-bold animate-pulse">
                                Loading synthetic stock market data...
                            </div>
                        ) : (
                            <div className="space-y-3 max-h-[520px] overflow-y-auto pr-1">
                                {filteredTickers.map((item) => {
                                    const isSelected = selectedCompany?.symbol === item.symbol;
                                    return (
                                        <Card
                                            key={item.symbol}
                                            className={`bg-white border-2 border-black transition-all cursor-pointer group ${
                                                isSelected
                                                    ? "ring-2 ring-blue-600 bg-blue-50/40 shadow-[4px_4px_0px_0px_rgba(37,99,235,1)]"
                                                    : "shadow-[3px_3px_0px_0px_rgba(0,0,0,1)] hover:translate-y-[1px] hover:shadow-[1px_1px_0px_0px_rgba(0,0,0,1)]"
                                            }`}
                                            onClick={() => handleSelectTicker(item)}
                                        >
                                            <Card.Content className="p-4">
                                                <div className="flex justify-between items-start mb-1">
                                                    <div>
                                                        <div className="flex items-center gap-2">
                                                            <Text className="font-black text-lg group-hover:text-blue-600 transition-colors">
                                                                {item.symbol}
                                                            </Text>
                                                            <Badge variant="outline" className="text-[10px] px-1.5 py-0 border border-black bg-gray-100">
                                                                {item.sector}
                                                            </Badge>
                                                        </div>
                                                        <Text className="text-gray-600 font-bold text-xs">{item.name}</Text>
                                                    </div>
                                                    <div className="text-right">
                                                        <Text className="font-bold text-base font-mono">
                                                            ${Number(item.current_price).toFixed(2)}
                                                        </Text>
                                                        <span className={`text-xs font-bold ${item.is_positive ? 'text-green-600' : 'text-red-600'}`}>
                                                            {item.is_positive ? '+' : ''}{item.change_percent}%
                                                        </span>
                                                    </div>
                                                </div>
                                                <Text className="text-gray-500 text-xs line-clamp-2 mt-1">
                                                    {item.description}
                                                </Text>
                                            </Card.Content>
                                        </Card>
                                    );
                                })}
                            </div>
                        )}
                    </div>
                </div>
            </div>
        </div>
    );
};

export default BuyStockPage;
