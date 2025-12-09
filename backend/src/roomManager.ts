import { Room, Player, ROOM_MAX_PLAYERS } from './types';
import { v4 as uuidv4 } from 'uuid';

export class RoomManager {
    private rooms: Map<string, Room> = new Map();

    createRoom(hostId: string, username: string, tournamentId?: string): Room {
        const roomId = uuidv4().slice(0, 6).toUpperCase();
        const newRoom: Room = {
            id: roomId,
            hostId,
            players: [{
                id: hostId,
                username,
                progress: 0,
                wpm: 0,
                finished: false
            }],
            status: 'waiting',
            text: "The quick brown fox jumps over the lazy dog. Programming is the art of telling another human what one wants the computer to do.",
            tournamentId
        };
        this.rooms.set(roomId, newRoom);
        return newRoom;
    }

    joinRoom(roomId: string, userId: string, username: string): Room | null {
        const room = this.rooms.get(roomId);
        if (!room) return null;
        if (room.status !== 'waiting') return null;
        if (room.players.length >= ROOM_MAX_PLAYERS) return null;

        room.players.push({
            id: userId,
            username,
            progress: 0,
            wpm: 0,
            finished: false
        });
        return room;
    }

    getRoom(roomId: string): Room | undefined {
        return this.rooms.get(roomId);
    }

    removePlayer(roomId: string, userId: string): Room | null {
        const room = this.rooms.get(roomId);
        if (!room) return null;

        room.players = room.players.filter(p => p.id !== userId);

        if (room.players.length === 0) {
            this.rooms.delete(roomId);
            return null;
        } else if (room.hostId === userId) {
            room.hostId = room.players[0].id;
        }
        return room;
    }
}
