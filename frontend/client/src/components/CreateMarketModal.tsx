import { useState } from 'react';
import { X, TrendingUp, HelpCircle } from 'lucide-react';

interface CreateMarketModalProps {
    isOpen: boolean;
    onClose: () => void;
    onCreate: (type: 'Binary' | 'Price', details: any, duration: number) => void;
}

export function CreateMarketModal({ isOpen, onClose, onCreate }: CreateMarketModalProps) {
    const [marketType, setMarketType] = useState<'Binary' | 'Price'>('Binary');
    const [question, setQuestion] = useState('');
    const [symbol, setSymbol] = useState('BTC');
    const [targetPrice, setTargetPrice] = useState('');
    const [duration, setDuration] = useState('60'); // minutes

    if (!isOpen) return null;

    const handleSubmit = (e: React.FormEvent) => {
        e.preventDefault();
        const details = marketType === 'Binary'
            ? { question }
            : { symbol, targetPrice: parseFloat(targetPrice) };

        onCreate(marketType, details, parseInt(duration));
        onClose();
    };

    return (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm p-4">
            <div className="bg-white dark:bg-gray-800 rounded-2xl w-full max-w-md shadow-xl border border-gray-200 dark:border-gray-700">
                <div className="flex items-center justify-between p-6 border-b border-gray-100 dark:border-gray-700">
                    <h2 className="text-xl font-bold text-gray-900 dark:text-white">Create Market</h2>
                    <button onClick={onClose} className="text-gray-400 hover:text-gray-500 dark:hover:text-gray-300">
                        <X className="w-6 h-6" />
                    </button>
                </div>

                <form onSubmit={handleSubmit} className="p-6 space-y-6">
                    {/* Market Type Selection */}
                    <div className="grid grid-cols-2 gap-3">
                        <button
                            type="button"
                            onClick={() => setMarketType('Binary')}
                            className={`p-4 rounded-xl border-2 flex flex-col items-center gap-2 transition-all ${marketType === 'Binary'
                                    ? 'border-emerald-500 bg-emerald-50 dark:bg-emerald-900/20 text-emerald-700 dark:text-emerald-400'
                                    : 'border-gray-200 dark:border-gray-700 text-gray-500 hover:border-emerald-200'
                                }`}
                        >
                            <HelpCircle className="w-6 h-6" />
                            <span className="font-bold text-sm">Yes/No Event</span>
                        </button>
                        <button
                            type="button"
                            onClick={() => setMarketType('Price')}
                            className={`p-4 rounded-xl border-2 flex flex-col items-center gap-2 transition-all ${marketType === 'Price'
                                    ? 'border-blue-500 bg-blue-50 dark:bg-blue-900/20 text-blue-700 dark:text-blue-400'
                                    : 'border-gray-200 dark:border-gray-700 text-gray-500 hover:border-blue-200'
                                }`}
                        >
                            <TrendingUp className="w-6 h-6" />
                            <span className="font-bold text-sm">Price Target</span>
                        </button>
                    </div>

                    {/* Dynamic Fields */}
                    <div className="space-y-4">
                        {marketType === 'Binary' ? (
                            <div>
                                <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                                    Question
                                </label>
                                <input
                                    type="text"
                                    required
                                    value={question}
                                    onChange={(e) => setQuestion(e.target.value)}
                                    placeholder="e.g., Will it rain in Tokyo tomorrow?"
                                    className="w-full px-4 py-2 rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-900 text-gray-900 dark:text-white focus:ring-2 focus:ring-emerald-500 focus:border-transparent"
                                />
                            </div>
                        ) : (
                            <div className="grid grid-cols-2 gap-4">
                                <div>
                                    <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                                        Symbol
                                    </label>
                                    <input
                                        type="text"
                                        required
                                        value={symbol}
                                        onChange={(e) => setSymbol(e.target.value.toUpperCase())}
                                        placeholder="BTC"
                                        className="w-full px-4 py-2 rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-900 text-gray-900 dark:text-white focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                                    />
                                </div>
                                <div>
                                    <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                                        Target Price ($)
                                    </label>
                                    <input
                                        type="number"
                                        required
                                        value={targetPrice}
                                        onChange={(e) => setTargetPrice(e.target.value)}
                                        placeholder="50000"
                                        className="w-full px-4 py-2 rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-900 text-gray-900 dark:text-white focus:ring-2 focus:ring-blue-500 focus:border-transparent"
                                    />
                                </div>
                            </div>
                        )}

                        <div>
                            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
                                Duration (Minutes)
                            </label>
                            <select
                                value={duration}
                                onChange={(e) => setDuration(e.target.value)}
                                className="w-full px-4 py-2 rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-900 text-gray-900 dark:text-white focus:ring-2 focus:ring-indigo-500 focus:border-transparent"
                            >
                                <option value="5">5 Minutes (Flash)</option>
                                <option value="15">15 Minutes</option>
                                <option value="60">1 Hour</option>
                                <option value="1440">24 Hours</option>
                            </select>
                        </div>
                    </div>

                    <button
                        type="submit"
                        className="w-full py-3 px-4 bg-gray-900 dark:bg-white text-white dark:text-gray-900 font-bold rounded-xl hover:bg-gray-800 dark:hover:bg-gray-100 transition-colors"
                    >
                        Create Market
                    </button>
                </form>
            </div>
        </div>
    );
}
