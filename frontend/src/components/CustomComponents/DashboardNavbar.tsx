import { useNavigate } from 'react-router-dom';
import { Button } from "@/components/retroui/Button";
import { Text } from "@/components/retroui/Text";
import { BarChart3, LogOut } from "lucide-react";

export const DashboardNavbar = () => {
    const navigate = useNavigate();

    return (
        <nav className="border-b-4 border-black bg-white px-6 py-4 sticky top-0 z-50">
            <div className="max-w-7xl mx-auto flex justify-between items-center">
                <div className="flex items-center gap-2 cursor-pointer" onClick={() => navigate('/landing_page')}>
                    <div className="bg-black text-white p-2 rounded-lg transform -rotate-3">
                        <BarChart3 className="w-6 h-6" />
                    </div>
                    <Text as="h3" className="text-2xl font-black tracking-tighter">RETRO<span className="text-blue-600">TRADE</span></Text>
                </div>
                <div className="flex gap-4">
                    <Button
                        variant="ghost"
                        onClick={() => navigate('/landing_page')}
                        className="font-bold flex items-center gap-2"
                    >
                        <LogOut className="w-4 h-4" /> Logout
                    </Button>
                </div>
            </div>
        </nav>
    );
};
