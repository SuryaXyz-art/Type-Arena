export interface Player {
    id: string;
    username: string;
    wpm: number;
    progress: number;
    finished: boolean;
}

export interface Room {
    id: string;
    hostId: string;
    players: Player[];
    text: string;
    isStarted: boolean;
    startTime: number | null;
}

export interface Tournament {
    id: string;
    hostId: string;
    players: string[]; // ids
    playerNames: Record<string, string>;
    maxPlayers: number;
    status: 'waiting' | 'started' | 'finished';
    bracket: Match[][];
}

export interface Match {
    player1: string | null;
    player2: string | null;
    winner: string | null;
    roomId: string | null;
}

export interface LeaderboardEntry {
    username: string;
    wpm: number;
}
