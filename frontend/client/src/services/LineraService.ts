
import { Client } from '@linera/client';

export class LineraService {
    private client: Client | null = null;
    // private tokenAppId: string | null = null;
    private marketAppId: string | null = null;
    // private oracleAppId: string | null = null;

    setClient(client: Client) {
        this.client = client;
    }

    setAppIds(_tokenAppId: string, marketAppId: string, _oracleAppId: string) {
        // this.tokenAppId = tokenAppId;
        this.marketAppId = marketAppId;
        // this.oracleAppId = oracleAppId;
    }

    private async getApplication(appId: string | null) {
        if (!this.client) {
            throw new Error("Linera client not initialized");
        }
        if (!appId) {
            throw new Error("Application ID not set");
        }
        return await this.client.application(appId);
    }

    async submitScore(roomId: string, wpm: number, timeMs: number) {
        console.log(`[Linera] Submitting score for room ${roomId}: ${wpm} WPM`);
        const application = await this.getApplication(this.marketAppId);
        const query = `mutation { submitResult(roomId: "${roomId}", wpm: ${wpm}, timeMs: ${timeMs}) }`;
        await application.query(query);
    }

    async createRoom(roomId: string, text: string) {
        console.log(`[Linera] Creating room ${roomId} with text length ${text.length}`);
        const application = await this.getApplication(this.marketAppId);
        const query = `mutation { createRoom(roomId: "${roomId}", text: "${text}") }`;
        await application.query(query);
    }

    async finishRoom(roomId: string) {
        console.log(`[Linera] Finishing room ${roomId}`);
        const application = await this.getApplication(this.marketAppId);
        const query = `mutation { finishRoom(roomId: "${roomId}") }`;
        await application.query(query);
    }

    async getRoom(roomId: string) {
        const application = await this.getApplication(this.marketAppId);
        const query = `{ room(roomId: "${roomId}") { id host players { address wpm finishTimeMs } isFinished } }`;
        const response = await application.query(query);
        return JSON.parse(response).data;
    }

    async getPlayerStats(address: string) {
        const application = await this.getApplication(this.marketAppId);
        const query = `{ playerStats(address: "${address}") { wins totalRaces bestWpm } }`;
        const response = await application.query(query);
        return JSON.parse(response).data;
    }
}

export const lineraService = new LineraService();
