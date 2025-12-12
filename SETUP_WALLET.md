# Setting Up Your Linera Wallet for Type Arena

To interact with the Type Arena application on the Linera network, you need to initialize a wallet, deploy the contract, and configure your frontend environment.

## Prerequisites
- [x] Rust Toolchain (Installed)
- [ ] **Linera CLI**: You must install the Linera CLI tools.
  ```powershell
  cargo install linera-service linera-storage-service
  ```
- [ ] **System PATH**: Ensure your Cargo bin directory is in your PATH.
  ```powershell
  $env:PATH += ";$env:USERPROFILE\.cargo\bin"
  ```

## Step 1: Initialize Your Wallet
Run the following command to create a new wallet with a default chain:
```bash
linera wallet init
```
This creates `wallet.json` and `wallet.db` in your current directory.

## Step 2: Deploy the Contract
Navigate to the project root and run the deployment script:
```powershell
.\deploy.ps1
```
This script will:
1. Build the WASM artifacts.
2. Publish the bytecode to the network.
3. Create the Type Arena application.
4. Output the **Application ID**.

## Step 3: Configure Frontend
1. Copy `frontend/client/.env.example` to `frontend/client/.env`.
2. Get your Default Chain ID:
   ```bash
   linera wallet show
   ```
   Copy the `Chain Id` from the output.
3. Open `frontend/client/.env` and update the values:
   ```ini
   VITE_CHAIN_ID=<Paste your Chain ID>
   VITE_TOKEN_APP_ID=<Paste Application ID from deploy.ps1 output>
   ```

## Step 4: Run the App
```bash
cd frontend/client
npm start
```
The application will now connect to your Linera chain using the specific IDs.
