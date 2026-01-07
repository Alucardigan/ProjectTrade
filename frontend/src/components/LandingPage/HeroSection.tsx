import { useNavigate } from 'react-router-dom';
import { Button } from "@/components/retroui/Button";
import { Text } from "@/components/retroui/Text";
import { Badge } from "@/components/retroui/Badge";
import { ArrowRight, Shield, Zap, TrendingUp } from "lucide-react";
import { MockPortfolioCard } from "./MockPortfolioCard";

export const HeroSection = () => {
    const navigate = useNavigate();

    return (
        <div className="flex flex-col lg:flex-row items-center gap-12 lg:gap-24">
            <div className="flex-1 space-y-8 text-center lg:text-left">
                <Badge variant="retro" className="px-4 py-1 text-sm border-2 border-black">
                    🚀 v2.0 is now live
                </Badge>

                <div className="space-y-4">
                    <Text as="h1" className="text-6xl lg:text-8xl font-black text-gray-900 leading-[0.9] tracking-tight">
                        TRADE LIKE <br />
                        <span className="text-transparent bg-clip-text bg-gradient-to-r from-blue-600 to-purple-600">IT'S 1999</span>
                    </Text>
                    <Text className="text-xl text-gray-600 font-medium max-w-xl mx-auto lg:mx-0 leading-relaxed">
                        Experience the thrill of the market with a nostalgic twist.
                        Real-time data, paper trading, and zero risk. All wrapped in a
                        beautiful neo-brutalist interface.
                    </Text>
                </div>

                <div className="flex flex-col sm:flex-row gap-4 justify-center lg:justify-start">
                    <Button
                        size="lg"
                        onClick={() => navigate('/')}
                        className="text-lg px-8 py-6 border-2 border-black shadow-[6px_6px_0px_0px_rgba(0,0,0,1)] hover:shadow-[3px_3px_0px_0px_rgba(0,0,0,1)] hover:translate-y-[3px] transition-all bg-yellow-400 text-black hover:bg-yellow-300"
                    >
                        Start Trading Now <ArrowRight className="ml-2 w-5 h-5" />
                    </Button>
                    <Button
                        size="lg"
                        variant="outline"
                        className="text-lg px-8 py-6 border-2 border-black shadow-[6px_6px_0px_0px_rgba(0,0,0,1)] hover:shadow-[3px_3px_0px_0px_rgba(0,0,0,1)] hover:translate-y-[3px] transition-all bg-white"
                    >
                        View Demo
                    </Button>
                </div>

                <div className="pt-8 flex items-center justify-center lg:justify-start gap-8 text-gray-500 font-bold text-sm uppercase tracking-widest">
                    <div className="flex items-center gap-2">
                        <Shield className="w-5 h-5" /> Secure
                    </div>
                    <div className="flex items-center gap-2">
                        <Zap className="w-5 h-5" /> Fast
                    </div>
                    <div className="flex items-center gap-2">
                        <TrendingUp className="w-5 h-5" /> Real-Time
                    </div>
                </div>
            </div>

            {/* Hero Image / Visual */}
            <div className="flex-1 w-full max-w-lg lg:max-w-none relative">
                <div className="absolute -top-12 -right-12 w-24 h-24 bg-yellow-400 rounded-full border-4 border-black z-0 hidden lg:block animate-bounce" style={{ animationDuration: '3s' }}></div>
                <div className="absolute -bottom-8 -left-8 w-16 h-16 bg-blue-400 rounded-full border-4 border-black z-0 hidden lg:block animate-bounce" style={{ animationDuration: '4s' }}></div>

                <MockPortfolioCard />
            </div>
        </div>
    );
};
