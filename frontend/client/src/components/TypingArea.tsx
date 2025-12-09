import { useState, useEffect, useRef } from 'react';

interface TypingAreaProps {
    text: string;
    startTime: number;
    onProgress: (progress: number, wpm: number) => void;
}

export function TypingArea({ text, startTime, onProgress }: TypingAreaProps) {
    const [input, setInput] = useState('');
    const [wpm, setWpm] = useState(0);
    const inputRef = useRef<HTMLTextAreaElement>(null);

    useEffect(() => {
        if (inputRef.current) {
            inputRef.current.focus();
        }
    }, []);

    const handleChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
        const value = e.target.value;
        setInput(value);

        // Calculate progress
        const correctChars = value.split('').filter((char, i) => char === text[i]).length;
        const progress = Math.min(100, (correctChars / text.length) * 100);

        // Calculate WPM
        const timeElapsed = (Date.now() - startTime) / 1000 / 60; // minutes
        const wordsTyped = value.length / 5;
        const currentWpm = timeElapsed > 0 ? Math.round(wordsTyped / timeElapsed) : 0;
        setWpm(currentWpm);

        onProgress(progress, currentWpm);
    };

    return (
        <div className="w-full max-w-4xl mx-auto p-6 bg-gray-800 rounded-xl shadow-2xl">
            <div className="mb-6 p-4 bg-gray-900 rounded-lg text-lg text-gray-300 font-mono leading-relaxed select-none">
                {text.split('').map((char, index) => {
                    let color = 'text-gray-500';
                    if (index < input.length) {
                        color = input[index] === char ? 'text-green-400' : 'text-red-500';
                    }
                    return <span key={index} className={color}>{char}</span>;
                })}
            </div>

            <textarea
                ref={inputRef}
                value={input}
                onChange={handleChange}
                className="w-full h-32 p-4 bg-gray-700 text-white rounded-lg focus:outline-none focus:ring-2 focus:ring-purple-500 font-mono text-lg"
                placeholder="Start typing..."
                spellCheck={false}
            />

            <div className="mt-4 flex justify-between items-center text-xl font-bold">
                <div className="text-purple-400">WPM: {wpm}</div>
                <div className="text-blue-400">Progress: {Math.round((input.length / text.length) * 100)}%</div>
            </div>
        </div>
    );
}
