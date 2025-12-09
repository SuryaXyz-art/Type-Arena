import { GraphQLClient, gql } from 'graphql-request';

const NODE_URL = "http://localhost:8080";
const CHAIN_ID = "e476187f6ddfeb9d588c7b45d3df334d5501d6499b3f9ad5595cae86cce16a65"; // Replace with actual Chain ID
const APPLICATION_ID = "e476187f6ddfeb9d588c7b45d3df334d5501d6499b3f9ad5595cae86cce16a65"; // Replace with actual Application ID

export class LineraService {
    private client: GraphQLClient;

    constructor() {
        // In Linera, the endpoint is typically /chains/<chain_id>/applications/<app_id>
        this.client = new GraphQLClient(`${NODE_URL}/chains/${CHAIN_ID}/applications/${APPLICATION_ID}`);
    }

    async submitScore(roomId: string, wpm: number, timeMs: number) {
        console.log(`[Linera] Submitting score for room ${roomId}: ${wpm} WPM`);

        const mutation = gql`
            mutation SubmitResult($roomId: String!, $wpm: Int!, $timeMs: Int!) {
                submitResult(roomId: $roomId, wpm: $wpm, timeMs: $timeMs)
            }
        `;

        try {
            await this.client.request(mutation, { roomId, wpm, timeMs });
        } catch (error) {
            console.error("Failed to submit score to Linera:", error);
            throw error;
        }
    }

    async createRoom(roomId: string) {
        console.log(`[Linera] Creating room ${roomId}`);
        const mutation = gql`
            mutation CreateRoom($roomId: String!) {
                createRoom(roomId: $roomId)
            }
        `;
        try {
            await this.client.request(mutation, { roomId });
        } catch (error) {
            console.error("Failed to create room on Linera:", error);
            throw error;
        }
    }

    async finishRoom(roomId: string) {
        console.log(`[Linera] Finishing room ${roomId}`);
        const mutation = gql`
            mutation FinishRoom($roomId: String!) {
                finishRoom(roomId: $roomId)
            }
        `;
        try {
            await this.client.request(mutation, { roomId });
        } catch (error) {
            console.error("Failed to finish room on Linera:", error);
            throw error;
        }
    }

    async getRoom(roomId: string) {
        const query = gql`
            query GetRoom($roomId: String!) {
                rooms(key: $roomId) {
                    id
                    host
                    players {
                        address
                        wpm
                        finishTimeMs
                    }
                    isFinished
                }
            }
         `;
        return await this.client.request(query, { roomId });
    }
}

export const lineraService = new LineraService();
