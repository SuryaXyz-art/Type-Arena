import fs from 'fs';
import path from 'path';

export interface HighScore {
    username: string;
    wpm: number;
    date: number;
}

const DATA_FILE = path.join(__dirname, '../leaderboard.json');

export class LeaderboardManager {
    private scores: HighScore[] = [];

    constructor() {
        this.load();
    }

    private load() {
        try {
            if (fs.existsSync(DATA_FILE)) {
                const data = fs.readFileSync(DATA_FILE, 'utf-8');
                this.scores = JSON.parse(data);
            }
        } catch (error) {
            console.error('Failed to load leaderboard:', error);
            this.scores = [];
        }
    }

    private save() {
        try {
            fs.writeFileSync(DATA_FILE, JSON.stringify(this.scores, null, 2));
        } catch (error) {
            console.error('Failed to save leaderboard:', error);
        }
    }

    addScore(username: string, wpm: number) {
        this.scores.push({ username, wpm, date: Date.now() });
        this.scores.sort((a, b) => b.wpm - a.wpm);
        if (this.scores.length > 50) {
            this.scores = this.scores.slice(0, 50);
        }
        this.save();
    }

    getTopScores(limit: number = 10): HighScore[] {
        return this.scores.slice(0, limit);
    }
}
