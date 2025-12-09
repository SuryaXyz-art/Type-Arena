import { Clock, DollarSign } from 'lucide-react';
import { PredictionModule } from './PredictionModule';

interface MarketCardProps {
    id: string;
    question: string;
    closesAt: number; // timestamp in ms
    totalPool: number;
    upPool: number;
    downPool: number;
    status: 'Open' | 'Locked' | 'Resolved' | 'Cancelled';
    onBet: (amount: number, prediction: 'Up' | 'Down') => void;
}

export function MarketCard({
    question,
    closesAt,
    totalPool,
    upPool,
    downPool,
    status,
    onBet,
}: MarketCardProps) {
    const timeLeft = Math.max(0, closesAt - Date.now());
    const minutesLeft = Math.floor(timeLeft / 60000);
    const secondsLeft = Math.floor((timeLeft % 60000) / 1000);

    const isExpired = timeLeft === 0;

    return (
        <div className="bg-white dark:bg-gray-800 rounded-xl overflow-hidden border border-gray-200 dark:border-gray-700 shadow-sm hover:shadow-md transition-shadow">
            <div className="p-6">
                <div className="flex justify-between items-start mb-4">
                    <h3 className="text-xl font-bold text-gray-900 dark:text-white flex-1 mr-4">
                        {question}
                    </h3>
                    <div className={`px-3 py-1 rounded-full text-xs font-bold ${status === 'Open' && !isExpired ? 'bg-emerald-100 text-emerald-800 dark:bg-emerald-900/30 dark:text-emerald-400' :
                            status === 'Resolved' ? 'bg-blue-100 text-blue-800 dark:bg-blue-900/30 dark:text-blue-400' :
                                'bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-300'
                        }`}>
                        {status === 'Open' && isExpired ? 'Closing...' : status}
                    </div>
                </div>

                <div className="flex items-center gap-4 mb-6 text-sm text-gray-500 dark:text-gray-400">
                    <div className="flex items-center gap-1">
                        <Clock className="w-4 h-4" />
                        <span>
                            {status === 'Open' && !isExpired
                                ? `${minutesLeft}m ${secondsLeft}s left`
                                : isExpired ? 'Ended' : 'Closed'}
                        </span>
                    </div>
                    <div className="flex items-center gap-1">
                        <DollarSign className="w-4 h-4" />
                        <span>Pool: {totalPool.toLocaleString()}</span>
                    </div>
                </div>

                {status === 'Open' && !isExpired && (
                    <PredictionModule
                        upPool={upPool}
                        downPool={downPool}
                        onBet={onBet}
                    />
                )}
            </div>
        </div>
    );
}
