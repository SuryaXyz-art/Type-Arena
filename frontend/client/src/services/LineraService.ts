
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

    async submitScore(roomId: string, wpm: number, timeMs: number, hostChainId: string) {
        console.log(`[Linera] Submitting score for room ${roomId}: ${wpm} WPM on host chain ${hostChainId}`);
        const application = await this.getApplication(this.marketAppId);
        const query = `mutation { submitResult(roomId: "${roomId}", wpm: ${wpm}, timeMs: ${timeMs}, hostChainId: "${hostChainId}") }`;
        await application.query(query);
    }

    async joinRoom(roomId: string, hostChainId: string) {
        console.log(`[Linera] Joining room ${roomId} on host chain ${hostChainId}`);
        const application = await this.getApplication(this.marketAppId);
        const query = `mutation { joinRoom(roomId: "${roomId}", hostChainId: "${hostChainId}") }`;
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
    async subscribeToEvents(callback: (event: any) => void) {
        if (!this.marketAppId) return;
        const application = await this.getApplication(this.marketAppId);
        // Note: This assumes the client supports subscription via this method or we need a proper subscription query
        // For standard Linera GraphQL:
        const subscriptionQuery = `subscription { events }`;
        // WARNING: 'service.rs' currently uses EmptySubscription. 
        // If this query fails, you may need to implement 'SubscriptionRoot' in 'service.rs' using 'async-graphql' 
        // or rely on the Linera Client's system notification stream instead of app-level GraphQL.
        // This part depends on the specific client implementation. 
        // If 'subscribe' returns an observable/async iterable:
        try {
            // @ts-ignore
            const subscription = await application.subscribe(subscriptionQuery);
            // @ts-ignore
            for await (const response of subscription) {
                if (response.data && response.data.events) {
                    callback(response.data.events);
                }
            }
        } catch (e) {
            console.error("Subscription failed/not supported:", e);
        }
    }
}

export const lineraService = new LineraService();
