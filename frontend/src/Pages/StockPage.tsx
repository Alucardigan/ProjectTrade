import { useState } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { useQuery, useMutation } from '@tanstack/react-query';
import { BarChart, Bar, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer } from 'recharts';
import { DashboardNavbar } from '@/components/CustomComponents/DashboardNavbar';
import { Card } from '@/components/retroui/Card';
import { Text } from '@/components/retroui/Text';
import { Button } from '@/components/retroui/Button';
import { Badge } from '@/components/retroui/Badge';
import { fetchTickerHistory, fetchTicker } from '@/api/ticker';
import { placeOrder } from '@/api/orderManagement';
import { OrderType } from '@/types/OrderType';
import { TrendingUp, AlertCircle, ArrowRight, Activity, TrendingDown } from 'lucide-react';

const TIMEFRAMES = [
  { label: '1D', value: 'day' },
  { label: '1M', value: 'month' },
  { label: '6M', value: 'half_year' },
  { label: '1Y', value: 'year' },
  { label: '5Y', value: 'five_year' },
  { label: 'ALL', value: 'all_years' }
];

const Candlestick = (props: any) => {
  const { x, y, width, height, payload } = props;
  const { open, close, high, low } = payload;
  
  const isUp = close >= open;
  const color = isUp ? '#10b981' : '#ef4444';
  
  if (high === low) {
     return <line x1={x + width / 2} y1={y} x2={x + width / 2} y2={y + height} stroke={color} strokeWidth={2} />;
  }

  const openPixel = y + ((high - open) / (high - low)) * height;
  const closePixel = y + ((high - close) / (high - low)) * height;
  
  const topBox = Math.min(openPixel, closePixel);
  const bottomBox = Math.max(openPixel, closePixel);
  const boxHeight = Math.max(bottomBox - topBox, 1);
  
  return (
    <g>
      <line x1={x + width / 2} y1={y} x2={x + width / 2} y2={y + height} stroke={color} strokeWidth={2} />
      <rect x={x} y={topBox} width={width} height={boxHeight} fill={color} stroke={color} />
    </g>
  );
};

const CustomTooltip = ({ active, payload, label }: any) => {
  if (active && payload && payload.length) {
    const data = payload[0].payload;
    return (
      <div className="bg-white border-4 border-black p-3 shadow-[4px_4px_0px_0px_rgba(0,0,0,1)]">
        <Text className="font-bold mb-2 text-gray-500">{label}</Text>
        <div className="space-y-1">
          <Text className="font-bold text-sm">Open: ${Number(data.open).toFixed(2)}</Text>
          <Text className="font-bold text-sm">High: ${Number(data.high).toFixed(2)}</Text>
          <Text className="font-bold text-sm">Low: ${Number(data.low).toFixed(2)}</Text>
          <Text className="font-bold text-sm">Close: ${Number(data.close).toFixed(2)}</Text>
        </div>
      </div>
    );
  }
  return null;
};

const StockPage = () => {
    const { ticker } = useParams();
    const navigate = useNavigate();
    const [timeframe, setTimeframe] = useState('month');
    const [quantity, setQuantity] = useState("");

    const { data: tickerData } = useQuery({
        queryKey: ['ticker', ticker],
        queryFn: () => fetchTicker(ticker || ''),
        enabled: !!ticker
    });

    const { data: history, isLoading } = useQuery({
        queryKey: ['tickerHistory', ticker, timeframe],
        queryFn: () => fetchTickerHistory(ticker || '', timeframe),
        enabled: !!ticker
    });

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

    const chartData = history?.map((d: any) => {
        const close = Number(d.close);
        const open = d.open != null ? Number(d.open) : close;
        const high = d.high != null ? Number(d.high) : close;
        const low = d.low != null ? Number(d.low) : close;
        
        return {
            date: new Date(d.date).toLocaleDateString(),
            open,
            close,
            high,
            low,
            lowHigh: [low, high]
        };
    }) || [];

    const currentPrice = tickerData ? Number(tickerData.close) : 0;
    const isPositive = chartData.length >= 2 ? chartData[chartData.length - 1].close >= chartData[0].close : true;
    const todayData = chartData.length > 0 ? chartData[chartData.length - 1] : null;
    const totalCost = currentPrice && quantity ? currentPrice * Number(quantity) : 0;

    const handleBuy = () => {
        if (!ticker || !quantity) return;
        mutation.mutate({
            ticker: ticker,
            quantity: Number(quantity),
            order_type: OrderType.Buy,
            price_buffer: 0
        });
    };

    return (
        <div className="min-h-screen bg-yellow-50/50 font-sans pb-12">
            <DashboardNavbar />
            <div className="max-w-7xl mx-auto p-6 md:p-12">
                <Button
                    variant="ghost"
                    onClick={() => navigate(-1)}
                    className="mb-6 text-gray-600 hover:text-black pl-0"
                >
                    ← Back
                </Button>
                
                <div className="flex items-end gap-4 mb-8">
                    <Text as="h1" className="text-5xl font-black text-gray-900 tracking-tight">{ticker}</Text>
                    <div className="flex items-center gap-2 mb-1">
                        <Text className="text-3xl font-bold text-gray-900">
                            ${currentPrice.toFixed(2)}
                        </Text>
                        <Badge variant={isPositive ? "success" : "destructive"} className="border-2 border-black ml-2 py-1">
                            {isPositive ? <TrendingUp className="w-4 h-4 mr-1" /> : <TrendingDown className="w-4 h-4 mr-1" />}
                            Live
                        </Badge>
                    </div>
                </div>

                <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
                    {/* LEFT COLUMN: Chart & Stats */}
                    <div className="lg:col-span-2 space-y-8">
                        {/* Chart Card */}
                        <Card className="bg-white/80 backdrop-blur-md rounded-xl border-4 border-black p-6 shadow-[8px_8px_0px_0px_rgba(0,0,0,1)] flex flex-col gap-6">
                            <div className="flex justify-between items-center flex-wrap gap-4">
                                <Text className="text-2xl font-black">Performance</Text>
                                <div className="flex bg-gray-100 p-1 rounded-lg border-2 border-black overflow-x-auto">
                                {TIMEFRAMES.map((tf) => (
                                    <button
                                        key={tf.value}
                                        onClick={() => setTimeframe(tf.value)}
                                        className={`px-3 py-1 text-sm font-bold rounded transition-all duration-200 ${
                                            timeframe === tf.value 
                                            ? 'bg-black text-white shadow-sm' 
                                            : 'text-gray-500 hover:text-black'
                                        }`}
                                    >
                                        {tf.label}
                                    </button>
                                ))}
                                </div>
                            </div>

                            <div className="h-[450px] w-full mt-4">
                                {isLoading ? (
                                    <div className="w-full h-full flex items-center justify-center animate-pulse bg-gray-50 rounded-lg border-2 border-dashed border-gray-300">
                                        <Text className="text-gray-400 font-bold">Loading chart data...</Text>
                                    </div>
                                ) : chartData.length === 0 ? (
                                    <div className="w-full h-full flex items-center justify-center bg-gray-50 rounded-lg border-2 border-dashed border-gray-300">
                                        <Text className="text-gray-400 font-bold">No historical data found.</Text>
                                    </div>
                                ) : (
                                    <ResponsiveContainer width="100%" height="100%">
                                        <BarChart data={chartData} margin={{ top: 10, right: 10, left: -20, bottom: 0 }}>
                                            <CartesianGrid strokeDasharray="3 3" vertical={false} stroke="#e5e7eb" />
                                            <XAxis dataKey="date" axisLine={false} tickLine={false} tick={{ fill: '#6b7280', fontSize: 12, fontWeight: 600 }} minTickGap={30} />
                                            <YAxis axisLine={false} tickLine={false} tick={{ fill: '#6b7280', fontSize: 12, fontWeight: 600 }} tickFormatter={(val: number) => `$${val.toFixed(0)}`} domain={['auto', 'auto']} />
                                            <Tooltip content={<CustomTooltip />} cursor={{ fill: 'rgba(0,0,0,0.05)' }} />
                                            <Bar dataKey="lowHigh" shape={<Candlestick />} />
                                        </BarChart>
                                    </ResponsiveContainer>
                                )}
                            </div>
                        </Card>

                        {/* Stats Grid */}
                        {todayData && (
                            <div>
                                <Text as="h3" className="text-xl font-bold text-gray-800 mb-4 flex items-center gap-2">
                                    <Activity className="w-5 h-5" /> Key Statistics
                                </Text>
                                <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
                                    <Card className="bg-white border-2 border-black shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] p-4 text-center">
                                        <Text className="text-sm font-bold text-gray-500 uppercase tracking-wider mb-1">Open</Text>
                                        <Text className="text-2xl font-black text-gray-900">${todayData.open.toFixed(2)}</Text>
                                    </Card>
                                    <Card className="bg-white border-2 border-black shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] p-4 text-center">
                                        <Text className="text-sm font-bold text-gray-500 uppercase tracking-wider mb-1">High</Text>
                                        <Text className="text-2xl font-black text-green-600">${todayData.high.toFixed(2)}</Text>
                                    </Card>
                                    <Card className="bg-white border-2 border-black shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] p-4 text-center">
                                        <Text className="text-sm font-bold text-gray-500 uppercase tracking-wider mb-1">Low</Text>
                                        <Text className="text-2xl font-black text-red-600">${todayData.low.toFixed(2)}</Text>
                                    </Card>
                                    <Card className="bg-white border-2 border-black shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] p-4 text-center">
                                        <Text className="text-sm font-bold text-gray-500 uppercase tracking-wider mb-1">Close</Text>
                                        <Text className="text-2xl font-black text-gray-900">${todayData.close.toFixed(2)}</Text>
                                    </Card>
                                </div>
                            </div>
                        )}
                    </div>

                    {/* RIGHT COLUMN: Trade Panel */}
                    <div className="lg:col-span-1">
                        <Card className="bg-white border-4 border-black shadow-[8px_8px_0px_0px_rgba(0,0,0,1)] sticky top-6">
                            <Card.Content className="p-6 space-y-6">
                                <Text as="h2" className="text-2xl font-black border-b-4 border-black pb-4">Trade {ticker}</Text>
                                
                                <div className="space-y-2">
                                    <label className="text-sm font-bold uppercase tracking-wider text-gray-500">Shares</label>
                                    <input
                                        type="number"
                                        value={quantity}
                                        onChange={(e) => setQuantity(e.target.value)}
                                        placeholder="0"
                                        min="1"
                                        className="w-full px-4 py-4 bg-gray-50 border-2 border-black rounded text-xl font-bold focus:outline-none focus:ring-2 focus:ring-blue-500 transition-all placeholder:text-gray-300"
                                    />
                                </div>

                                <div className="bg-blue-50 p-4 rounded border-2 border-black border-dashed space-y-3">
                                    <div className="flex justify-between items-center">
                                        <Text className="text-gray-600 font-bold">Market Price</Text>
                                        <Text className="font-bold">${currentPrice.toFixed(2)}</Text>
                                    </div>
                                    <div className="border-t-2 border-black border-dashed"></div>
                                    <div className="flex justify-between items-end">
                                        <Text className="font-bold uppercase">Total Cost</Text>
                                        <Text className="text-2xl font-black text-blue-600">
                                            ${totalCost.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })}
                                        </Text>
                                    </div>
                                </div>

                                <Button
                                    onClick={handleBuy}
                                    disabled={!quantity || mutation.isPending}
                                    className="w-full py-6 text-xl bg-green-500 hover:bg-green-600 text-white border-2 border-black shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] hover:shadow-[2px_2px_0px_0px_rgba(0,0,0,1)] hover:translate-y-[2px] transition-all disabled:opacity-50 disabled:cursor-not-allowed"
                                >
                                    {mutation.isPending ? "Processing..." : (
                                        <span className="flex items-center justify-center gap-2">
                                            Buy Asset <ArrowRight className="w-6 h-6" />
                                        </span>
                                    )}
                                </Button>

                                <div className="flex items-start gap-2 text-gray-500 text-sm font-bold bg-gray-50 p-3 rounded border-2 border-gray-200">
                                    <AlertCircle className="w-5 h-5 flex-shrink-0 text-gray-400" />
                                    Market orders are executed immediately at the best available price.
                                </div>
                            </Card.Content>
                        </Card>
                    </div>
                </div>
            </div>
        </div>
    );
};

export default StockPage;
