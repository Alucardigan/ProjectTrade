import { Card } from "@/components/retroui/Card";
import { Text } from "@/components/retroui/Text";
import { Monitor, Zap, Shield } from "lucide-react";

export const FeaturesSection = () => {
    return (
        <div className="mt-32 grid grid-cols-1 md:grid-cols-3 gap-8">
            <Card className="bg-white border-4 border-black shadow-[8px_8px_0px_0px_rgba(0,0,0,1)] hover:-translate-y-2 transition-all">
                <Card.Content className="p-8 text-center space-y-4">
                    <div className="w-16 h-16 mx-auto bg-green-100 rounded-full border-4 border-black flex items-center justify-center">
                        <Monitor className="w-8 h-8 text-green-700" />
                    </div>
                    <Text as="h3" className="text-2xl font-black">Retro UI</Text>
                    <Text className="text-gray-600">
                        A design system that takes you back to the golden age of the internet, but with modern performance.
                    </Text>
                </Card.Content>
            </Card>

            <Card className="bg-white border-4 border-black shadow-[8px_8px_0px_0px_rgba(0,0,0,1)] hover:-translate-y-2 transition-all">
                <Card.Content className="p-8 text-center space-y-4">
                    <div className="w-16 h-16 mx-auto bg-purple-100 rounded-full border-4 border-black flex items-center justify-center">
                        <Zap className="w-8 h-8 text-purple-700" />
                    </div>
                    <Text as="h3" className="text-2xl font-black">Lightning Fast</Text>
                    <Text className="text-gray-600">
                        Built with Rust and React for blazing fast execution and real-time updates.
                    </Text>
                </Card.Content>
            </Card>

            <Card className="bg-white border-4 border-black shadow-[8px_8px_0px_0px_rgba(0,0,0,1)] hover:-translate-y-2 transition-all">
                <Card.Content className="p-8 text-center space-y-4">
                    <div className="w-16 h-16 mx-auto bg-red-100 rounded-full border-4 border-black flex items-center justify-center">
                        <Shield className="w-8 h-8 text-red-700" />
                    </div>
                    <Text as="h3" className="text-2xl font-black">Risk Free</Text>
                    <Text className="text-gray-600">
                        Practice your trading strategies with paper money before you risk a single cent.
                    </Text>
                </Card.Content>
            </Card>
        </div>
    );
};
