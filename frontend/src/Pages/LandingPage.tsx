import { Navbar } from "@/components/LandingPage/Navbar";
import { HeroSection } from "@/components/LandingPage/HeroSection";
import { FeaturesSection } from "@/components/LandingPage/FeaturesSection";

export default function LandingPage() {
    return (
        <div className="min-h-screen bg-yellow-50/50 font-sans selection:bg-yellow-200">
            <Navbar />
            <main className="max-w-7xl mx-auto px-6 pt-12 pb-24">
                <HeroSection />
                <FeaturesSection />
            </main>
        </div>
    );
}
