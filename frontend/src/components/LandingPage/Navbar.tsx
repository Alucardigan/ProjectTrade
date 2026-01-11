import { useNavigate } from 'react-router-dom';
import { Button } from "@/components/retroui/Button";
import { Text } from "@/components/retroui/Text";
import { BarChart3 } from "lucide-react";

export const Navbar = () => {
    const navigate = useNavigate();

    return (
        <nav className="border-b-4 border-black bg-white px-6 py-4 sticky top-0 z-50">
            <div className="max-w-7xl mx-auto flex justify-between items-center">
                <div className="flex items-center gap-2">
                    <div className="bg-black text-white p-2 rounded-lg transform -rotate-3">
                        <BarChart3 className="w-6 h-6" />
                    </div>
                    <Text as="h3" className="text-2xl font-black tracking-tighter">RE<span className="text-blue-600">TRADE</span></Text>
                </div>
                <div className="flex gap-4">
                    <Button variant="ghost" className="hidden md:flex font-bold">Features</Button>
                    <Button variant="ghost" className="hidden md:flex font-bold">Pricing</Button>
                    <Button
                        onClick={() => navigate('/')}
                        className="font-bold border-2 border-black shadow-[4px_4px_0px_0px_rgba(0,0,0,1)] hover:shadow-[2px_2px_0px_0px_rgba(0,0,0,1)] hover:translate-y-[2px] transition-all"
                    >
                        Login
                    </Button>
                </div>
            </div>
        </nav>
    );
};
