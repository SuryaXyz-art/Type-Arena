# Linera Wallet Integration Example

This directory contains a self-contained example of how to integrate a Linera wallet (using MetaMask) into a web application.

## How to Use

1.  **Open `index.html` in your browser:** You can open the `index.html` file directly in your web browser.
2.  **Replace Placeholders:** Before you can use this example, you need to replace the following placeholders in `index.html`:
    *   `REPLACE_WITH_YOUR_APPLICATION_ID`: Replace this with the application ID of your deployed Linera application (e.g., the "counter" application).
3.  **Connect Your Wallet:** Click the "Connect Wallet" button to connect your MetaMask wallet.
4.  **Interact with the Application:** Once connected, you will see your owner address and chain ID. You can then interact with the application (in this example, by clicking the "Increment Counter" button).

## Adapting to Your Application

You can adapt this code to your own frontend application by following these steps:

1.  **Copy the `@linera` packages:** Copy the `@linera/client` and `@linera/metamask` directories into your project.
2.  **Add the `importmap`:** Add the `importmap` from the `index.html` to your main HTML file to ensure that the Linera packages are correctly resolved.
3.  **Add the "Connect Wallet" logic:** Add a "Connect Wallet" button and the associated JavaScript code to your application.
4.  **Create the `linera.Client`:** Once the wallet is connected, create a `linera.Client` object with the signer and wallet from MetaMask.
5.  **Interact with your application:** Use the `client.application()` method to get a reference to your Linera application and then use the `query()` method to send queries and mutations.

## Connecting to Conway Testnet

To connect to the Conway testnet, you need to:

1.  **Get a Faucet:** Find a faucet for the Conway testnet. The `frontend/README.md` of the "Flash Markets" project mentions the chain ID, but not a faucet URL. You may need to ask the Linera community for a faucet URL.
2.  **Update `faucetUrl`:** In `index.html`, change the `faucetUrl` variable to the URL of the Conway testnet faucet.
3.  **Deploy your application to Conway:** You will need to deploy your Linera application to the Conway testnet and get its application ID.
4.  **Update `applicationId`:** In `index.html`, replace `REPLACE_WITH_YOUR_APPLICATION_ID` with the application ID of your application on the Conway testnet.
