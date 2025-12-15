import { Signer } from '@linera/metamask';
import { Client, Faucet } from '@linera/client';

class WalletService {
  public client: Client | null = null;
  private signer: Signer | null = null;

  async connect() {
    // Client.initialize();
    this.signer = new Signer();

    // The rest of the client setup will be done in the component
    // after we have the faucet and other info.
    return this.signer;
  }

  async createClient(faucetUrl: string, _chainId: string) {
    if (!this.signer) {
      throw new Error('Signer not initialized. Call connect() first.');
    }

    const faucet = new Faucet(faucetUrl);
    const wallet = await faucet.createWallet();
    const owner = await this.signer.address();
    await faucet.claimChain(wallet, owner);

    this.client = new Client(wallet, this.signer, false);
    return this.client;
  }
}

export const walletService = new WalletService();
