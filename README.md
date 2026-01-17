# Type Arena - Wave 6 Submission

## 🚨 Judge Quick Start (The "Green" Path)

You can play the live version of Type Arena immediately on Testnet Conway without installing any local dependencies.

**[Link to Live Demo - Placeholder]** 
*(Please deploy the `frontend/client/dist` folder to Vercel/Netlify after running `./deploy_testnet.sh` and update this link)*

## 🎥 Video Demo
[Insert Link to YouTube/Loom Video Here]

## 📸 Screenshots
![Game Lobby](/path/to/screenshot1.png)
*(Replace with actual screenshots of the lobby)*

![Race in Progress](/path/to/screenshot2.png)
*(Replace with actual screenshots of the racing screen)*

---

## 🛠️ Local Setup (Docker)

If you prefer to run the project locally, we have provided a Docker container that handles all Rust and Linera dependencies.

### Prerequisites
* Docker installed and running.

### Instructions
1. Run the container:
   ```bash
   docker build -t type-arena .
   docker run -p 8080:8080 type-arena
   ```
2. Open `http://localhost:8080` in your browser.

*Note: The Docker version serves the frontend and compiles the contracts. To interact with the chain, ensure you have the Linera Wallet extension installed and connected to Testnet Conway.*

---

## 🌐 Deploying to Testnet Conway (Manual)

If you wish to deploy your own instance of the game to the testnet:

1. **Install Linera:** Ensure `linera` CLI is in your PATH.
2. **Run the Deployment Script:**
   ```bash
   ./deploy_testnet.sh
   ```
   This script will:
   * Initialize a wallet against `faucet.testnet-conway.linera.net`.
   * Compile the Rust smart contracts.
   * Publish bytecode and create the application on the public testnet.
   * Print the `APP_ID` and `CHAIN_ID`.

3. **Run the Frontend:**
   ```bash
   cd frontend/client
   npm install
   npm run dev
   ```

---

## ✅ Technical Checklist (Wave 6)

* **Testnet Conway:** Fully integrated. The app does not rely on a local `linera net up` network.
* **Headers:** COOP/COEP headers are strictly enforced in `vite.config.ts` and `run.sh` to support SharedArrayBuffer.
* **Dependencies:** All `Cargo.toml` dependencies use pinned crate versions (e.g., `linera-sdk = "0.15.7"`), no local paths.
* **Cross-Platform:** Includes un.sh (Bash) and Dockerfile for Linux/MacOS compatibility, replacing the previous Windows-only PowerShell scripts.

## 🚀 Technical Highlights (Judge Criteria)
### ✅ Microchains Architecture
Type Arena uses a **User-Centric Chain** model. 
- **Host Chain:** Manages globally visible Tournament/Room state.
- **User Chains:** Players interact with the game from their own microchains.
- **Cross-Chain Messaging:** When a player joins a room or submits a result from their chain, the contract automatically routes a **Cross-Chain Message** (`JoinRoom`, `SubmitResult`) to the Host Chain, ensuring scalability without congestion.

### ✅ Real-Time Features
The application uses **Linera Events** for sub-second updates.
- **Push-based:** No polling.
- **Events:** `RoomCreated`, `PlayerJoined`, `ResultSubmitted`, `RoomFinished` are emitted to the `events` stream, allowing the frontend to react instantly via GraphQL subscriptions.

## 🏗️ Architecture Note

*   **Settlement:** All race results are cryptographically verified and stored on Linera.
*   **Performance:** Cross-chain messaging ensures that intense gameplay (typing races) does not bloat the main coordination chain.
