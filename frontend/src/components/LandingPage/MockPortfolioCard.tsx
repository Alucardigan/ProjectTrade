import { Card } from "@/components/retroui/Card";
import { Text } from "@/components/retroui/Text";
import { Badge } from "@/components/retroui/Badge";
import { TrendingUp } from "lucide-react";

export const MockPortfolioCard = () => {
    return (
        <Card className="relative z-10 bg-white border-4 border-black shadow-[12px_12px_0px_0px_rgba(0,0,0,1)] transform rotate-2 hover:rotate-0 transition-all duration-500">
            <Card.Header className="border-b-4 border-black bg-gray-50 p-4 flex justify-between items-center">
                <div className="flex gap-2">
                    <div className="w-3 h-3 rounded-full bg-red-500 border border-black"></div>
                    <div className="w-3 h-3 rounded-full bg-yellow-500 border border-black"></div>
                    <div className="w-3 h-3 rounded-full bg-green-500 border border-black"></div>
                </div>
                <div className="px-3 py-1 bg-white border-2 border-black rounded text-xs font-mono font-bold">
                    portfolio.exe
                </div>
            </Card.Header>
            <Card.Content className="p-6 space-y-6">
                <div className="flex justify-between items-end">
                    <div>
                        <Text className="text-xs font-bold text-gray-500 uppercase">Total Balance</Text>
                        <Text className="text-4xl font-black">$124,592.45</Text>
                    </div>
                    <Badge variant="success" className="border-2 border-black text-sm px-3 py-1">
                        +12.5%
                    </Badge>
                </div>
                <div className="space-y-3">
                    {[1, 2, 3].map((i) => (
                        <div key={i} className="flex items-center gap-4 p-3 bg-gray-50 border-2 border-black rounded hover:bg-blue-50 transition-colors cursor-pointer">
                            <div className={`w-10 h-10 rounded border-2 border-black flex items-center justify-center ${i === 1 ? 'bg-red-100' : i === 2 ? 'bg-green-100' : 'bg-purple-100'}`}>
                                <TrendingUp className="w-5 h-5" />
                            </div>
                            <div className="flex-1">
                                <div className="h-2 w-24 bg-gray-800 rounded mb-1"></div>
                                <div className="h-2 w-16 bg-gray-400 rounded"></div>
                            </div>
                            <div className="text-right">
                                <div className="h-2 w-12 bg-gray-800 rounded mb-1 ml-auto"></div>
                            </div>
                        </div>
                    ))}
                </div>
            </Card.Content>
        </Card>
    );
};
