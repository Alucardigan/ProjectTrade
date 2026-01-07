import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogFooter } from "@/components/ui/dialog";
import { Button } from "@/components/retroui/Button";
import { Text } from "@/components/retroui/Text";
import { useState } from "react";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { placeOrder } from "../../api/orderManagement";
import { OrderType } from "../../types/OrderType";
import { Loader2, AlertCircle } from "lucide-react";

interface SellModalProps {
    isOpen: boolean;
    onClose: () => void;
    ticker: string;
    maxQuantity: number;
    currentPrice: number;
}

export const SellModal = ({ isOpen, onClose, ticker, maxQuantity, currentPrice }: SellModalProps) => {
    const [quantity, setQuantity] = useState("");
    const queryClient = useQueryClient();

    const mutation = useMutation({
        mutationFn: placeOrder,
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: ['portfolio'] });
            queryClient.invalidateQueries({ queryKey: ['accountBalance'] });
            onClose();
            setQuantity("");
        },
        onError: (error) => {
            console.error("Failed to sell asset:", error);
            alert("Failed to place sell order. Please try again.");
        }
    });

    const handleSell = () => {
        if (!quantity) return;
        mutation.mutate({
            symbol: ticker,
            quantity: Number(quantity),
            order_type: OrderType.Sell,
            price_buffer: 0
        });
    };

    const estimatedTotal = Number(quantity) * currentPrice;

    return (
        <Dialog open={isOpen} onOpenChange={onClose}>
            <DialogContent className="sm:max-w-md bg-white border-4 border-black shadow-[8px_8px_0px_0px_rgba(0,0,0,1)] p-0 gap-0 overflow-hidden">
                <DialogHeader className="p-6 bg-red-50 border-b-4 border-black">
                    <DialogTitle className="text-2xl font-black uppercase tracking-tight flex items-center gap-2">
                        Sell {ticker}
                    </DialogTitle>
                    <Text className="text-gray-600 font-medium">
                        Current Price: <span className="font-bold text-black">${currentPrice.toFixed(2)}</span>
                    </Text>
                </DialogHeader>

                <div className="p-6 space-y-6">
                    <div className="space-y-2">
                        <div className="flex justify-between">
                            <label className="text-sm font-bold uppercase tracking-wider text-gray-500">Quantity to Sell</label>
                            <span className="text-xs font-bold text-blue-600 cursor-pointer hover:underline" onClick={() => setQuantity(maxQuantity.toString())}>
                                Max: {maxQuantity}
                            </span>
                        </div>
                        <input
                            type="number"
                            value={quantity}
                            onChange={(e) => setQuantity(e.target.value)}
                            placeholder="0"
                            max={maxQuantity}
                            min="0"
                            className="w-full px-4 py-3 bg-gray-50 border-2 border-black rounded text-lg font-bold focus:outline-none focus:ring-2 focus:ring-red-500 focus:border-transparent transition-all"
                        />
                    </div>

                    <div className="bg-gray-50 p-4 rounded border-2 border-black border-dashed">
                        <div className="flex justify-between items-end">
                            <Text className="text-sm font-bold uppercase text-gray-500">Estimated Value</Text>
                            <Text className="text-2xl font-black text-green-600">
                                ${estimatedTotal.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })}
                            </Text>
                        </div>
                    </div>

                    <div className="flex items-start gap-2 text-xs text-gray-500 font-medium bg-yellow-50 p-3 rounded border border-yellow-200">
                        <AlertCircle className="w-4 h-4 text-yellow-600 shrink-0 mt-0.5" />
                        <p>Funds will be added to your available balance immediately upon execution.</p>
                    </div>
                </div>

                <DialogFooter className="p-6 bg-gray-50 border-t-2 border-black">
                    <div className="flex gap-3 w-full">
                        <Button variant="ghost" onClick={onClose} className="flex-1">
                            Cancel
                        </Button>
                        <Button
                            onClick={handleSell}
                            disabled={!quantity || Number(quantity) <= 0 || Number(quantity) > maxQuantity || mutation.isPending}
                            className="flex-1 bg-red-500 hover:bg-red-600 text-white border-2 border-black shadow-[2px_2px_0px_0px_rgba(0,0,0,1)] hover:translate-y-[1px] hover:shadow-[1px_1px_0px_0px_rgba(0,0,0,1)] transition-all disabled:opacity-50 disabled:cursor-not-allowed"
                        >
                            {mutation.isPending ? <Loader2 className="w-4 h-4 animate-spin" /> : "Confirm Sell"}
                        </Button>
                    </div>
                </DialogFooter>
            </DialogContent>
        </Dialog>
    );
};
