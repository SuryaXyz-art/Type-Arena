import { Wallet, Zap } from 'lucide-react';

interface HeaderProps {
    balance: number;
    address: string;
}

export function Header({ balance, address }: HeaderProps) {
    return (
        <header className="bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700 sticky top-0 z-50">
            <div className="container mx-auto px-4 h-16 flex items-center justify-between">
                <div className="flex items-center gap-2">
                    <div className="bg-emerald-500 p-2 rounded-lg">
                        <Zap className="w-5 h-5 text-white" />
                    </div>
                    <span className="text-xl font-bold bg-gradient-to-r from-emerald-600 to-emerald-400 bg-clip-text text-transparent">
                        Flash Markets
                    </span>
                </div>

                <div className="flex items-center gap-4">
                    <div className="hidden md:flex items-center gap-2 px-4 py-2 bg-gray-50 dark:bg-gray-900 rounded-lg border border-gray-200 dark:border-gray-700">
                        <Wallet className="w-4 h-4 text-gray-500" />
                        <span className="font-mono text-sm text-gray-700 dark:text-gray-300">
                            {address.slice(0, 6)}...{address.slice(-4)}
                        </span>
                    </div>

                    <div className="flex items-center gap-2 px-4 py-2 bg-emerald-50 dark:bg-emerald-900/20 rounded-lg border border-emerald-100 dark:border-emerald-900/30">
                        <span className="font-bold text-emerald-600 dark:text-emerald-400">
                            {balance.toLocaleString()}
                        </span>
                        <span className="text-xs font-medium text-emerald-600/70 dark:text-emerald-400/70">
                            TOKENS
                        </span>
                    </div>
                </div>
            </div>
        </header>
    );
}
