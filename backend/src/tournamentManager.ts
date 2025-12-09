import { v4 as uuidv4 } from 'uuid';

interface Tournament {
    id: string;
    hostId: string;
    players: string[];
    playerNames: { [id: string]: string };
    maxPlayers: number;
    currentRound: number;
    status: 'waiting' | 'active' | 'finished';
    bracket: Match[][]; // Array of rounds, each round is array of Matches
}

interface Match {
    id: string; // Internal match ID
    player1: string | null;
    player2: string | null;
    winner: string | null;
    roomId?: string;
}

export class TournamentManager {
    private tournaments: Map<string, Tournament> = new Map();

    createTournament(hostId: string, maxPlayers: number = 8): Tournament {
        const id = uuidv4().slice(0, 6).toUpperCase();
        const tournament: Tournament = {
            id,
            hostId,
            players: [hostId],
            playerNames: {},
            maxPlayers,
            currentRound: 1,
            status: 'waiting',
            bracket: []
        };
        this.tournaments.set(id, tournament);
        return tournament;
    }

    joinTournament(id: string, userId: string, username: string): Tournament | null {
        const tournament = this.tournaments.get(id);
        if (!tournament || tournament.status !== 'waiting') return null;
        if (tournament.players.length >= tournament.maxPlayers) return null;

        if (!tournament.players.includes(userId)) {
            tournament.players.push(userId);
            tournament.playerNames[userId] = username;
        }
        return tournament;
    }

    getTournament(id: string): Tournament | undefined {
        return this.tournaments.get(id);
    }

    startTournament(id: string): Tournament | null {
        const tournament = this.tournaments.get(id);
        if (!tournament) return null;

        tournament.status = 'active';

        // Generate Round 1
        const matches: Match[] = [];
        const players = [...tournament.players];

        // Shuffle players? Nah, join order for now.
        for (let i = 0; i < players.length; i += 2) {
            matches.push({
                id: uuidv4(),
                player1: players[i] || null,
                player2: players[i + 1] || null,
                winner: null
            });
        }
        tournament.bracket.push(matches);
        return tournament;
    }

    // Called when a race finishes
    recordMatchWinner(tournamentId: string, roomId: string, winnerId: string): Tournament | null {
        const tournament = this.tournaments.get(tournamentId);
        if (!tournament) return null;

        // Find match with this roomId
        const currentRoundMatches = tournament.bracket[tournament.bracket.length - 1];
        const match = currentRoundMatches.find(m => m.roomId === roomId);

        if (match) {
            match.winner = winnerId;

            // Check if round finished
            if (currentRoundMatches.every(m => m.winner !== null || (m.player2 === null && m.player1 !== null))) {
                this.generateNextRound(tournament);
            }
        }
        return tournament;
    }

    private generateNextRound(tournament: Tournament) {
        const previousRound = tournament.bracket[tournament.bracket.length - 1];
        const winners = previousRound.map(m => m.winner || m.player1); // Auto-win if no opponent?

        if (winners.length <= 1) {
            tournament.status = 'finished';
            return;
        }

        const nextRoundMatches: Match[] = [];
        for (let i = 0; i < winners.length; i += 2) {
            nextRoundMatches.push({
                id: uuidv4(),
                player1: winners[i] || null,
                player2: winners[i + 1] || null,
                winner: null
            });
        }
        tournament.bracket.push(nextRoundMatches);
        tournament.currentRound += 1;
    }

    // Assign room ID to a match
    assignRoomToMatch(tournamentId: string, matchId: string, roomId: string) {
        const tournament = this.tournaments.get(tournamentId);
        if (tournament) {
            for (const round of tournament.bracket) {
                const match = round.find(m => m.id === matchId);
                if (match) {
                    match.roomId = roomId;
                    break;
                }
            }
        }
    }
}
