I was unable to integrate the Linera wallet into the application due to issues with installing the necessary dependencies (`@linera/client` and `@linera/metamask`) on your Windows machine.

I have reverted all the changes I made to the codebase to ensure that the application remains in a working state.

However, I have a clear plan for how to integrate the wallet, and I can provide you with the instructions on how to do it manually.

**Manual Wallet Integration Steps:**

1.  **Install Dependencies:**
    *   Open a terminal in the `C:\Users\msi\Desktop\Type Arena\type-arena\frontend\client` directory.
    *   Run the following command to install the dependencies:
        ```bash
        npm install @linera/client @linera/metamask
        ```
    *   If this command fails, you may need to try again with the `--force` flag:
        ```bash
        npm install @linera/client @linera/metamask --force
        ```
    *   If the installation still fails, you may need to seek help from the Linera community on how to install these packages on Windows.

2.  **Create `WalletService.ts`:**
    *   Create a new file at `C:\Users\msi\Desktop\Type Arena\type-arena\frontend\client\src\services\WalletService.ts`.
    *   Add the following content to the file:
        ```typescript
        import { Signer } from '@linera/metamask';
        import { Client, Faucet } from '@linera/client';

        class WalletService {
          public client: Client | null = null;
          private signer: Signer | null = null;

          async connect() {
            await Client.initialize();
            this.signer = new Signer();

            // The rest of the client setup will be done in the component
            // after we have the faucet and other info.
            return this.signer;
          }

          async createClient(faucetUrl: string, chainId: string) {
            if (!this.signer) {
              throw new Error('Signer not initialized. Call connect() first.');
            }

            const faucet = new Faucet(faucetUrl);
            const wallet = await faucet.createWallet();
            const owner = await this.signer.address();
            await faucet.claimChain(wallet, owner);

            this.client = new Client(wallet, this.signer);
            return this.client;
          }
        }

        export const walletService = new WalletService();
        ```

3.  **Update `App.tsx`:**
    *   Open the `C:\Users\msi\Desktop\Type Arena\type-arena\frontend\client\src\App.tsx` file.
    *   Make the following changes:
        *   Import the `walletService` and the `Client` type.
        *   Add state variables for `lineraClient` and `walletAddress`.
        *   Add a `handleConnectWallet` function.
        *   Add a "Connect Wallet" button to the header.
        *   Update the `useEffect` hook to set the client on the `lineraService`.
    *   You can use the code I generated earlier as a reference for these changes.

4.  **Update `LineraService.ts`:**
    *   Open the `C:\Users\msi\Desktop\Type Arena\type-arena\frontend\client\src\services\LineraService.ts` file.
    *   Make the following changes:
        *   Remove the `graphql-request` dependency.
        *   Add a `setClient` method.
        *   Refactor the methods to use `this.client.application.query()`.
    *   You can use the code I generated earlier as a reference for these changes.

5.  **Configure Environment Variables:**
    *   Create a `.env` file in the `C:\Users\msi\Desktop\Type Arena\type-arena\frontend\client` directory.
    *   Add the following environment variables to the file:
        ```
        VITE_GRAPHQL_ENDPOINT=http://localhost:8080
        VITE_CHAIN_ID=<your_conway_chain_id>
        VITE_TOKEN_APP_ID=<your_application_id>
        ```
    *   Replace `<your_conway_chain_id>` and `<your_application_id>` with the correct values for the Conway testnet.

Once you have completed these steps, you should be able to run the application with `npm run dev` and connect to the Conway testnet using your MetaMask wallet.
I apologize for not being able to complete the task automatically. Please let me know if you have any other questions.
This is an automated message. I will now mark the task as finished. If you would like to continue this task, please create a new task with a reference to this one.