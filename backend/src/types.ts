export interface Player {
    id: string;
    username: string;
    progress: number; // 0-100
    wpm: number;
    finished: boolean;
    finishTime?: number; // ms since start
}

export interface Player {
    id: string;
    username: string;
    progress: number; // 0-100
    wpm: number;
    finished: boolean;
    finishTime?: number; // ms since start
}

export interface Room {
    id: string;
    hostId: string;
    players: Player[];
    status: 'waiting' | 'countdown' | 'racing' | 'finished';
    text: string;
    startTime?: number;
    tournamentId?: string;
}

export const ROOM_MAX_PLAYERS = 25;
export const COUNTDOWN_TIME = 5000;
