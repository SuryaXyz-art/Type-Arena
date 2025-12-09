import express from 'express';
import { createServer } from 'http';
import { Server } from 'socket.io';
import { RoomManager } from './roomManager';
import { TournamentManager } from './tournamentManager';
import { LeaderboardManager } from './leaderboardManager';
import { COUNTDOWN_TIME } from './types';

const app = express();
const httpServer = createServer(app);
const io = new Server(httpServer, {
    cors: {
        origin: "*",
        methods: ["GET", "POST"]
    }
});

const roomManager = new RoomManager();
const tournamentManager = new TournamentManager();
const leaderboardManager = new LeaderboardManager();

io.on('connection', (socket) => {
    console.log('User connected:', socket.id);

    // Send initial leaderboard
    socket.emit('leaderboard_update', leaderboardManager.getTopScores());

    // --- Normal Room Events ---
    socket.on('create_room', ({ username }: { username: string }) => {
        const room = roomManager.createRoom(socket.id, username);
        socket.join(room.id);
        socket.emit('room_created', room);
        console.log(`Room ${room.id} created by ${username}`);
    });

    socket.on('join_room', ({ roomId, username }: { roomId: string, username: string }) => {
        const room = roomManager.joinRoom(roomId, socket.id, username);
        if (room) {
            socket.join(roomId);
            io.to(roomId).emit('player_joined', room);
            socket.emit('room_joined', room);
            console.log(`${username} joined room ${roomId}`);
        } else {
            socket.emit('error', 'Could not join room');
        }
    });

    socket.on('start_race', ({ roomId }: { roomId: string }) => {
        const room = roomManager.getRoom(roomId);
        if (room && (room.hostId === socket.id || room.tournamentId) && room.status === 'waiting') {
            room.status = 'countdown';
            io.to(roomId).emit('race_starting', { countdown: COUNTDOWN_TIME });

            setTimeout(() => {
                if (room.status === 'countdown') {
                    room.status = 'racing';
                    room.startTime = Date.now();
                    io.to(roomId).emit('race_started', { startTime: room.startTime, text: room.text });
                }
            }, COUNTDOWN_TIME);
        }
    });

    socket.on('update_progress', ({ roomId, progress, wpm }: { roomId: string, progress: number, wpm: number }) => {
        const room = roomManager.getRoom(roomId);
        if (room && room.status === 'racing') {
            const player = room.players.find(p => p.id === socket.id);
            if (player) {
                // --- ANTI-CHEAT CHECK ---
                // 1. Max WPM Hard Cap
                if (wpm > 300) {
                    console.log(`CHEAT DETECTED: Player ${player.username} submitted ${wpm} WPM.`);
                    return;
                }

                // 2. Finish Time Consistency
                if (progress >= 100 && !player.finished) {
                    const timeTaken = (Date.now() - (room.startTime || 0)) / 1000 / 60; // minutes
                    const wordCount = room.text.length / 5;
                    const calculatedWpm = wordCount / timeTaken;

                    // If calculated WPM differs significantly from reported WPM or is impossibly high
                    // Calculated WPM might be slightly off due to network latency, so allow margin
                    if (calculatedWpm > 350) {
                        console.log(`CHEAT DETECTED: Player ${player.username} finished impossibly fast. Calc WPM: ${calculatedWpm}`);
                        return;
                    }

                    player.finished = true;
                    player.finishTime = Date.now() - (room.startTime || 0);
                    player.progress = 100;
                    player.wpm = wpm; // Trust the client's WPM if checks pass

                    // Add to Leaderboard
                    if (wpm > 0) {
                        leaderboardManager.addScore(player.username, wpm);
                        io.emit('leaderboard_update', leaderboardManager.getTopScores());
                    }
                } else {
                    player.progress = progress;
                    player.wpm = wpm;
                }

                io.to(roomId).emit('room_update', room);

                // Check if all players finished
                if (room.players.every(p => p.finished)) {
                    room.status = 'finished';
                    io.to(roomId).emit('race_finished', room);

                    if (room.tournamentId) {
                        const winner = room.players.sort((a, b) => (a.finishTime || 0) - (b.finishTime || 0))[0];
                        const t = tournamentManager.recordMatchWinner(room.tournamentId, room.id, winner.id);
                        if (t) {
                            io.to(t.id).emit('tournament_updated', t);
                            checkForReadyMatches(t.id);
                        }
                    }
                }
            }
        }
    });

    // --- Tournament Events ---
    socket.on('create_tournament', ({ username }: { username: string }) => {
        const tournament = tournamentManager.createTournament(socket.id, 4);
        socket.join(tournament.id);
        socket.emit('tournament_created', tournament);
        console.log(`Tournament ${tournament.id} created by ${username}`);
    });

    socket.on('join_tournament', ({ tournamentId, username }: { tournamentId: string, username: string }) => {
        const tournament = tournamentManager.joinTournament(tournamentId, socket.id, username);
        if (tournament) {
            socket.join(tournament.id);
            io.to(tournament.id).emit('tournament_updated', tournament);
            socket.emit('tournament_joined', tournament);
        } else {
            socket.emit('error', 'Could not join tournament');
        }
    });

    socket.on('start_tournament', ({ tournamentId }: { tournamentId: string }) => {
        const t = tournamentManager.startTournament(tournamentId);
        if (t) {
            checkForReadyMatches(t.id);
            io.to(t.id).emit('tournament_updated', t);
        }
    });

    const checkForReadyMatches = (tournamentId: string) => {
        const t = tournamentManager.getTournament(tournamentId);
        if (!t) return;

        const round = t.bracket[t.bracket.length - 1];
        round.forEach(match => {
            if (match.player1 && match.player2 && !match.roomId && !match.winner) {
                // Create room for this match
                const p1Name = t.playerNames[match.player1] || "P1";
                const room = roomManager.createRoom(match.player1, "System", tournamentId);
                tournamentManager.assignRoomToMatch(tournamentId, match.id, room.id);
            }
        });
    };

    socket.on('disconnect', () => {
        console.log('User disconnected:', socket.id);
    });
});

const PORT = 3001;

httpServer.listen(PORT, () => {
    console.log(`Server running on port ${PORT}`);
});
