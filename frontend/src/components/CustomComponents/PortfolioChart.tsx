import { useState } from 'react';
import { useQuery } from '@tanstack/react-query';
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer } from 'recharts';
import { Text } from '@/components/retroui/Text';
import { fetchPortfolioHistory } from '@/api/portfolio';

const TIMEFRAMES = [
  { label: '1D', value: 'day' },
  { label: '1M', value: 'month' },
  { label: '6M', value: 'half_year' },
  { label: '1Y', value: 'year' },
  { label: '5Y', value: 'five_year' },
  { label: 'ALL', value: 'all_years' }
];

export const PortfolioChart = () => {
  const [timeframe, setTimeframe] = useState('month');

  const { data: history, isLoading } = useQuery({
    queryKey: ['portfolioHistory', timeframe],
    queryFn: () => fetchPortfolioHistory(timeframe)
  });

  // Calculate if the portfolio is up or down over this timeframe
  const isUp = history && history.length > 0
    ? Number(history[history.length - 1].total_value) >= Number(history[0].total_value)
    : true;

  const color = isUp ? '#10b981' : '#ef4444';
  const gradientId = `colorGradient_${isUp ? 'up' : 'down'}`;

  // Format data for recharts
  let chartData = history?.map((d: any) => ({
    date: new Date(d.date).toLocaleDateString(),
    value: Number(d.total_value)
  })) || [];

  // If there's only one data point (e.g. sparse mock data), duplicate it to draw a flat horizontal line
  if (chartData.length === 1) {
    chartData = [
      { ...chartData[0], date: 'Start' },
      { ...chartData[0], date: 'End' }
    ];
  }

  return (
    <div className="bg-white/80 backdrop-blur-md rounded-xl border-4 border-black p-6 shadow-[8px_8px_0px_0px_rgba(0,0,0,1)] flex flex-col gap-6">
      <div className="flex justify-between items-center">
        <Text className="text-2xl font-black">Performance</Text>

        {/* Timeframe Toggles */}
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

      <div className="h-[400px] w-full mt-4">
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
            <LineChart data={chartData} margin={{ top: 10, right: 10, left: -20, bottom: 0 }}>
              <CartesianGrid strokeDasharray="3 3" vertical={false} stroke="#e5e7eb" />
              <XAxis
                dataKey="date"
                axisLine={false}
                tickLine={false}
                tick={{ fill: '#6b7280', fontSize: 12, fontWeight: 600 }}
                minTickGap={30}
              />
              <YAxis
                axisLine={false}
                tickLine={false}
                tick={{ fill: '#6b7280', fontSize: 12, fontWeight: 600 }}
                tickFormatter={(val: number) => Intl.NumberFormat('en-US', { notation: 'compact', maximumFractionDigits: 1 }).format(val)}
                domain={['auto', 'auto']}
              />
              <Tooltip
                contentStyle={{
                  backgroundColor: '#ffffff',
                  border: '3px solid #000000',
                  boxShadow: '4px 4px 0px 0px rgba(0,0,0,1)',
                  borderRadius: '8px',
                  fontWeight: 'bold',
                }}
                itemStyle={{ color: '#000000', fontWeight: 'bold' }}
                formatter={(value: number) => [`$${value.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })}`, 'Portfolio Value']}
                labelStyle={{ color: '#6b7280', marginBottom: '4px' }}
              />
              <Line
                type="monotone"
                dataKey="value"
                stroke={color}
                strokeWidth={4}
                dot={false}
                activeDot={{ r: 8, strokeWidth: 0 }}
                animationDuration={1000}
              />
            </LineChart>
          </ResponsiveContainer>
        )}
      </div>
    </div>
  );
};
