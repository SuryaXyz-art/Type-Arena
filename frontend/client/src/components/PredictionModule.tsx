import { useState } from 'react';
import { TrendingUp, TrendingDown } from 'lucide-react';

interface PredictionModuleProps {
    upPool: number;
    downPool: number;
    onBet: (amount: number, prediction: 'Up' | 'Down') => void;
    disabled?: boolean;
}

export function PredictionModule({ upPool, downPool, onBet, disabled }: PredictionModuleProps) {
    const [amount, setAmount] = useState('');
    const [selectedPrediction, setSelectedPrediction] = useState<'Up' | 'Down' | null>(null);

    const calculatePotentialProfit = () => {
        const betAmount = parseFloat(amount) || 0;
        if (betAmount === 0 || !selectedPrediction) return 0;

        const losingPool = selectedPrediction === 'Up' ? downPool : upPool;
        const winningPool = selectedPrediction === 'Up' ? upPool : downPool;

        // Avoid division by zero if winning pool is empty (it will be at least betAmount after bet)
        const newWinningPool = winningPool + betAmount;

        // Profit = (bet / new_winning_pool) * losing_pool * 0.95 (5% fee)
        const profitShare = (betAmount / newWinningPool) * losingPool * 0.95;

        return profitShare;
    };

    const potentialProfit = calculatePotentialProfit();

    const handleBet = () => {
        if (selectedPrediction && amount) {
            onBet(parseFloat(amount), selectedPrediction);
            setAmount('');
            setSelectedPrediction(null);
        }
    };

    return (
        <div className="bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-xl p-6 space-y-6 shadow-sm">
            <div>
                <h3 className="text-lg font-bold text-gray-900 dark:text-white mb-4">Place Your Bet</h3>

                <div className="grid grid-cols-2 gap-3 mb-6">
                    <button
                        onClick={() => setSelectedPrediction('Up')}
                        className={`p-4 rounded-lg border-2 transition-all ${selectedPrediction === 'Up'
                                ? 'border-emerald-500 bg-emerald-50 dark:bg-emerald-900/20'
                                : 'border-gray-200 dark:border-gray-700 hover:border-emerald-500'
                            }`}
                    >
                        <div className="flex items-center justify-center gap-2 mb-2">
                            <TrendingUp className="w-5 h-5 text-emerald-600 dark:text-emerald-400" />
                            <span className="font-bold text-emerald-600 dark:text-emerald-400">UP</span>
                        </div>
                        <div className="text-sm text-gray-500 dark:text-gray-400">
                            Pool: {upPool.toLocaleString()}
                        </div>
                    </button>

                    <button
                        onClick={() => setSelectedPrediction('Down')}
                        className={`p-4 rounded-lg border-2 transition-all ${selectedPrediction === 'Down'
                                ? 'border-ruby bg-rose-50 dark:bg-rose-900/20'
                                : 'border-gray-200 dark:border-gray-700 hover:border-ruby'
                            }`}
                    >
                        <div className="flex items-center justify-center gap-2 mb-2">
                            <TrendingDown className="w-5 h-5 text-ruby dark:text-rose-400" />
                            <span className="font-bold text-ruby dark:text-rose-400">DOWN</span>
                        </div>
                        <div className="text-sm text-gray-500 dark:text-gray-400">
                            Pool: {downPool.toLocaleString()}
                        </div>
                    </button>
                </div>

                <div className="space-y-4">
                    <div>
                        <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
                            Bet Amount (Tokens)
                        </label>
                        <input
                            type="number"
                            value={amount}
                            onChange={(e) => setAmount(e.target.value)}
                            placeholder="0.00"
                            className="w-full px-4 py-3 bg-gray-50 dark:bg-gray-900 border border-gray-200 dark:border-gray-700 rounded-lg text-gray-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-emerald-500"
                        />
                    </div>

                    {amount && selectedPrediction && (
                        <div className="p-4 bg-emerald-50 dark:bg-emerald-900/10 border border-emerald-100 dark:border-emerald-900/30 rounded-lg">
                            <div className="flex items-center justify-between mb-2">
                                <span className="text-sm text-gray-500 dark:text-gray-400">Your bet:</span>
                                <span className="font-medium text-gray-900 dark:text-white">
                                    {parseFloat(amount).toLocaleString()} {selectedPrediction.toUpperCase()}
                                </span>
                            </div>
                            <div className="flex items-center justify-between">
                                <span className="text-sm text-gray-500 dark:text-gray-400">Potential profit:</span>
                                <span className="font-bold text-emerald-600 dark:text-emerald-400">
                                    ~{potentialProfit.toLocaleString(undefined, { maximumFractionDigits: 2 })}
                                </span>
                            </div>
                        </div>
                    )}

                    <button
                        onClick={handleBet}
                        disabled={disabled || !amount || !selectedPrediction}
                        className="w-full py-3 bg-emerald-600 hover:bg-emerald-700 disabled:bg-gray-300 dark:disabled:bg-gray-700 disabled:cursor-not-allowed text-white font-bold rounded-lg transition-colors"
                    >
                        Place Bet
                    </button>
                </div>
            </div>
        </div>
    );
}
