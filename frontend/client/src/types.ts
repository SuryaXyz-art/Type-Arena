export interface Room {
  id: string;
  hostId: string;
  players: Player[];
  text: string;
}

export interface Player {
  id: string;
  username: string;
  progress: number;
  wpm: number;
  finished: boolean;
}

export interface Tournament {
  id: string;
  hostId: string;
  players: string[];
  playerNames: { [key: string]: string };
  bracket: any[];
  status: 'waiting' | 'racing' | 'finished';
  maxPlayers: number;
}

export interface LeaderboardEntry {
  username: string;
  wpm: number;
}

export interface Config {
  chainId: string;
  tokenAppId: string;
  marketAppId: string;
  oracleAppId: string;
}