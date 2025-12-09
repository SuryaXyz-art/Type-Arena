# ⚡ Flash Markets

**Ultra-Fast Micro Prediction Markets on Linera Microchains**

> **🔴 LIVE ON TESTNET CONWAY**

> * **Chain ID:** `ff869722e5434effbdcb533eae9979085f0ee8283aa711a9c2501838683ff54f`

> * **Market App:** `1cbf188bf36951902f7d80b6410a27e558f981fe6f98c1e24b01426491bde0da`

> * **Oracle App:** `7303f1197240f8b869fd92c31d06fc29de982c85acdd703ba0679d89390e59a4`

> * **Token App:** `fce5a571299bf5868ae7fedbc8ed74df460b47788594e87e966bde6dde211246`

Real-time prediction markets that resolve in minutes, not days - showcasing Linera's instant finality and real-time capabilities.

<img src="./docs/flash-markets-architecture.png" width="700" alt="Flash Markets Architecture">

## 🚀 Quick Start

### Prerequisites
- Docker & Docker Compose
- Rust 1.86.0
- Linera CLI

### One-Command Deployment

```bash
git clone https://github.com/YOUR_USERNAME/flash-markets
cd flash-markets
docker compose up -d --build
```

Wait for initialization (~2-3 minutes), then open:
- **Frontend:** http://localhost:3000
- **GraphQL:** http://localhost:8080/graphql

## 📖 Overview

Flash Markets is a decentralized prediction market platform where users can create and bet on outcomes that resolve in **minutes, not days** - something only possible on Linera's real-time blockchain.

### Key Features

- ⚡ **Ultra-Fast Markets** - Markets resolve in 1-60 minutes
- 🎯 **Instant Finality** - Bets confirmed instantly, no waiting for blocks
- 🤖 **Automated Resolution** - Oracle-based price feeds auto-resolve markets
- 💰 **Fair Payouts** - Algorithmic payout distribution (95% to winners, 5% platform fee)
- 🔗 **Multi-Chain Architecture** - Leverages Linera's unique microchain design
- 📊 **Real-Time Updates** - Live market data via event streaming

### Example Markets

- "Will BTC price be higher in 5 minutes?" (auto-resolves)
- "Will ETH go above $3000 in the next hour?" (auto-resolves)
- "Will Player A win this game round?" (manual resolution)
- "Will the next Ethereum block have >200 transactions?" (verifiable)

## 🏗️ Architecture

Flash Markets uses Linera's multi-chain architecture with three cooperating applications:

```
Master Chain (Admin)
    ↓
Market Chains (Active markets)
    ├── Market App (creates markets, accepts bets)
    ├── Token App (manages betting tokens)
    └── Oracle App (price feeds, auto-resolution)
    ↓
User Chains (Personal balances & bets)
```

### Three-Application System

#### 1. **Token App** - Token Management
- User balance tracking
- Daily bonus system (24h cooldown)
- Transfer & mint operations
- Cross-app balance calls

#### 2. **Market App** - Core Prediction Logic
- Market creation & lifecycle
- Bet placement & validation
- Payout calculation & distribution
- Market cancellation & refunds

#### 3. **Oracle App** - Price Feeds & Resolution
- Submit price feeds
- Auto-resolve markets by price
- Manual resolution (admin)
- Future: AI integration for complex markets

## 🎮 How It Works

### Creating a Market

```rust
// Example: Create a 5-minute BTC price prediction market
mutation {
  createMarket(
    marketType: {
      pricePrediction: {
        symbol: "BTC",
        targetPrice: 50000
      }
    },
    durationMinutes: 5
  )
}
```

### Placing a Bet

```rust
mutation {
  placeBet(
    marketId: 1,
    prediction: Up,    // or Down
    amount: "1000000"  // 1 token = 1,000,000 atto
  )
}
```

### Market Resolution Flow

1. **Market Closes** - After duration expires
2. **Oracle Checks Price** - Gets latest price feed
3. **Outcome Determined** - Price vs. target price
4. **Winners Calculated** - Based on pool ratios
5. **Payouts Distributed** - Automatic via smart contract

### Payout Formula

```
winner_payout = (bet_amount / winning_pool) * total_pool * 0.95
platform_fee = total_pool * 0.05
```

Example:
- Total pool: 100 tokens
- UP bets: 30 tokens (3 users)
- DOWN bets: 70 tokens (7 users)
- Outcome: UP wins
- Each UP bettor gets: (their_bet / 30) * 100 * 0.95 = ~3.17x return

## 💻 Project Structure

```
flash-markets/
├── abi/                      # Shared types & utilities
│   └── src/
│       └── lib.rs            # Market, Bet, Outcome types
│
├── token/                    # Token management application
│   └── src/
│       ├── lib.rs            # ABI definition
│       ├── state.rs          # Balance & bonus state
│       ├── contract.rs       # Token operations
│       └── service.rs        # GraphQL queries
│
├── market/                   # Core prediction market
│   └── src/
│       ├── lib.rs            # ABI definition
│       ├── state.rs          # Markets & bets state
│       ├── contract.rs       # Market operations
│       └── service.rs        # GraphQL queries
│
├── oracle/                   # Price feeds & resolution
│   └── src/
│       ├── lib.rs            # ABI definition
│       ├── state.rs          # Price feed state
│       ├── contract.rs       # Oracle operations
│       └── service.rs        # GraphQL queries
│
├── Cargo.toml                # Workspace configuration
├── rust-toolchain.toml       # Rust 1.86.0
├── docker-compose.yml        # Easy deployment
└── README.md                 # This file
```

## 🛠️ Building from Source

### Install Rust Target

```bash
rustup target add wasm32-unknown-unknown
```

### Build All Applications

```bash
# Build entire workspace
cargo build --release --target wasm32-unknown-unknown

# Or build specific apps
cargo build -p token --release --target wasm32-unknown-unknown
cargo build -p market --release --target wasm32-unknown-unknown
cargo build -p oracle --release --target wasm32-unknown-unknown
```

### Expected Output

```
target/wasm32-unknown-unknown/release/
├── token_contract.wasm
├── token_service.wasm
├── market_contract.wasm
├── market_service.wasm
├── oracle_contract.wasm
└── oracle_service.wasm
```

## 🧪 Testing Locally

### Start Linera Node

```bash
linera net up
```

### Create Applications

```bash
# Publish token app
linera project publish-and-create token \
  --path token

# Publish market app
linera project publish-and-create market \
  --path market

# Publish oracle app
linera project publish-and-create oracle \
  --path oracle
```

### Start GraphQL Service

```bash
linera service --port 8080
```

### Test Queries

```graphql
# Get token balance
query {
  accounts {
    entry(key: "User:YOUR_ADDRESS") {
      value
    }
  }
}

# List all markets
query {
  markets {
    keys
  }
}

# Get market details
query {
  markets {
    entry(key: "1") {
      value {
        id
        status
        totalPool
        upPool
        downPool
      }
    }
  }
}
```

## 🎯 GraphQL API

### Token Operations

```graphql
# Get balance
query {
  accounts { entry(key: "User:ADDR") { value } }
}

# Claim daily bonus
mutation {
  claimBonus(owner: "User:ADDR")
}

# Transfer tokens
mutation {
  transfer(from: "User:ADDR1", to: "User:ADDR2", amount: "1000000")
}
```

### Market Operations

```graphql
# Create market
mutation {
  createMarket(
    marketType: { binaryEvent: { question: "Will it rain?" } },
    durationMinutes: 10
  )
}

# Place bet
mutation {
  placeBet(marketId: 1, prediction: Up, amount: "1000000")
}

# Resolve market (admin/oracle)
mutation {
  resolveMarket(marketId: 1, outcome: Up)
}

# Claim winnings
mutation {
  claimWinnings(marketId: 1)
}
```

### Oracle Operations

```graphql
# Submit price
mutation {
  submitPrice(symbol: "BTC", price: 50000)
}

# Resolve by price
mutation {
  resolveMarketByPrice(
    marketId: 1,
    symbol: "BTC",
    targetPrice: 49000
  )
}
```

## 🔗 Cross-Application Communication

Flash Markets demonstrates Linera's cross-application messaging:

### Market → Token (Balance Check)

```rust
let balance_response: TokenResponse = self.runtime
    .call_application(
        true,
        &params.token_app,
        &TokenOperation::Balance { owner: bettor }
    )
    .await?;
```

### Oracle → Market (Auto-Resolve)

```rust
self.runtime
    .call_application(
        true,
        &params.market_app,
        &MarketOperation::ResolveMarket { market_id, outcome }
    )
    .await?;
```

## 🌊 Wave 3 Changelog

**Initial Release (Wave 3 Submission)**

- ✅ Three-application architecture (Token, Market, Oracle)
- ✅ Complete market lifecycle (create, bet, resolve, claim)
- ✅ Daily bonus system with 24h cooldown
- ✅ Oracle price feed integration
- ✅ Automatic market resolution
- ✅ Fair payout distribution algorithm
- ✅ Docker deployment setup
- ✅ GraphQL API for all operations
- ✅ Deployed to Conway Testnet

**Features Implemented:**
- Market creation with duration validation
- Bet placement with balance checks
- Market cancellation with refunds
- Payout calculation & distribution
- Platform fee collection (5%)
- Daily token bonuses

## 🎓 Key Linera Features Used

### 1. Multi-Chain Architecture
Different chains for different purposes:
- Master chain for admin operations
- Market chains for active markets
- User chains for personal state

### 2. Cross-Application Calls
```rust
self.runtime.call_application(auth, &app_id, &operation).await
```

### 3. Real-Time State Updates
Markets resolve instantly when time expires - no block waiting!

### 4. Event Streaming (Ready for Wave 4)
```rust
self.runtime.emit_event(MarketResolved { market_id, outcome });
```

### 5. Account Owner System
```rust
let signer = self.runtime.authenticated_signer()?;
```

## 📊 Constants & Configuration

```rust
// Minimum bet: 0.1 tokens
const MIN_BET_AMOUNT: u128 = 100_000;

// Market creation fee: 1 token
const MARKET_CREATION_FEE: u128 = 1_000_000;

// Platform fee: 5%
const PLATFORM_FEE_PERCENT: u8 = 5;

// Min duration: 1 minute
const MIN_MARKET_DURATION_MICROS: u64 = 60_000_000;

// Max duration: 24 hours
const MAX_MARKET_DURATION_MICROS: u64 = 86_400_000_000;

// Daily bonus cooldown: 24 hours
const ONE_DAY_MICROS: u64 = 86_400_000_000;
```

## 🚀 Roadmap & Future Development

We're committed to making Flash Markets the premier prediction market platform on Linera. Here's our development roadmap:

### 🎯 Phase 1: UI/UX Enhancement (Post-Wave 3)
**Goal: Professional, production-ready frontend**

- [ ] **Wallet Integration** - Browser wallet support for seamless transactions
- [ ] **Polished React UI** - Complete redesign with modern components
- [ ] **Real-Time Updates** - Event streaming for live market data (no polling)
- [ ] **Mobile-First Design** - Fully responsive across all devices
- [ ] **Market Analytics Dashboard** - Charts, trends, historical data
- [ ] **User Profiles** - Track betting history, win rates, leaderboards

### 🤖 Phase 2: AI & Automation (Wave 4-5)
**Goal: Intelligent market creation and resolution**

- [ ] **AI Market Creation** - LLM-powered market suggestions
- [ ] **Automated Oracle** - AI validates and resolves complex markets
- [ ] **Sentiment Analysis** - Social media integration for market trends
- [ ] **Smart Notifications** - AI predicts when users should bet
- [ ] **Price Prediction Models** - ML models for better odds
- [ ] **Auto-Hedging** - AI-driven risk management for users

### 📈 Phase 3: Market Expansion (Wave 5-6)
**Goal: Diverse market types and use cases**

- [ ] **Sports Betting** - Live sports markets with real-time odds
- [ ] **NFT Markets** - Predict NFT floor prices
- [ ] **Governance Markets** - Predict DAO proposal outcomes
- [ ] **Weather Markets** - Temperature, rainfall predictions
- [ ] **Blockchain Metrics** - Block times, gas prices, TVL predictions
- [ ] **Custom Markets** - User-generated market types

### 💎 Phase 4: DeFi Integration (Wave 6+)
**Goal: Advanced financial features**

- [ ] **Liquidity Pools** - Automated market making (AMM)
- [ ] **Market Maker Rewards** - Incentivize liquidity providers
- [ ] **Staking** - Stake tokens for boosted rewards
- [ ] **Cross-Chain Bridges** - Trade with other blockchain assets
- [ ] **Derivatives** - Options and futures on market outcomes
- [ ] **Insurance Markets** - Hedge against market volatility

### 🌐 Phase 5: Social & Community (Long-term)
**Goal: Build the prediction market community**

- [ ] **Social Trading** - Follow top traders, copy bets
- [ ] **Referral Program** - Earn rewards for bringing users
- [ ] **Tournament Mode** - Compete for prizes in weekly contests
- [ ] **Creator Economy** - Market creators earn from trading fees
- [ ] **DAO Governance** - Community votes on platform changes
- [ ] **Educational Content** - Tutorials, guides, market analysis

### 🔐 Phase 6: Enterprise Features (Future)
**Goal: B2B and institutional use**

- [ ] **Private Markets** - Company-specific prediction markets
- [ ] **API Access** - Programmatic trading for institutions
- [ ] **White-Label Solution** - Deploy Flash Markets for custom use cases
- [ ] **Compliance Tools** - KYC/AML for regulated markets
- [ ] **Advanced Analytics API** - Market data for research
- [ ] **Custom Oracle Integration** - Enterprise data feeds

### 🎓 Why This Roadmap Matters

**Linera's Real-Time Advantage:**
Every feature above leverages Linera's instant finality and real-time capabilities - things impossible on traditional blockchains.

**Progressive Decentralization:**
We start with a functional MVP and gradually decentralize through DAO governance and community features.

**Sustainable Growth:**
Platform fees (5%) fund development, creating a self-sustaining ecosystem.

**Vision:**
Flash Markets will become the Uniswap of prediction markets - the default platform where people bet on anything, instantly.

## 🏆 Why Flash Markets Wins

### ✅ Multiple Applications (3)
Following the pattern of ALL winning projects

### ✅ Token Economy
Complete token system with bonuses and transfers

### ✅ Prediction Markets
Exactly what the buildathon requested

### ✅ Real-Time
Markets resolve in minutes - showcases Linera's speed

### ✅ Cross-Chain Messaging
Extensive use of cross-app calls

### ✅ Scalable Architecture
Easy to add new market types

### ✅ Clear Use Case
Fast markets that other blockchains can't do

### ✅ Professional Code
Clean, well-documented, follows best practices

## 📝 Technical Details

### State Management

All applications use Linera's View system:

```rust
#[derive(RootView)]
#[view(context = ViewStorageContext)]
pub struct TokenState {
    pub accounts: MapView<AccountOwner, Amount>,
    pub daily_bonuses: MapView<AccountOwner, DailyBonus>,
    pub total_supply: RegisterView<Amount>,
}
```

### Operation Pattern

All operations follow this pattern:

```rust
async fn execute_operation(&mut self, operation: Self::Operation) -> Self::Response {
    match operation {
        Operation::DoSomething { params } => {
            // 1. Validate
            // 2. Check permissions
            // 3. Update state
            // 4. Call other apps if needed
            // 5. Return response
        }
    }
}
```

## 🐛 Troubleshooting

### Build Fails

```bash
# Ensure correct Rust version
rustup override set 1.86.0

# Clean and rebuild
cargo clean
cargo build --release --target wasm32-unknown-unknown
```

### GraphQL Not Working

```bash
# Check if service is running
linera service --port 8080

# Test with curl
curl -X POST http://localhost:8080/graphql \
  -H "Content-Type: application/json" \
  -d '{"query":"query { accounts { keys } }"}'
```

### Docker Issues

```bash
# Rebuild containers
docker compose down
docker compose up -d --build

# Check logs
docker compose logs -f
```



## 🙏 Acknowledgments

- **Linera Team** - For the amazing real-time blockchain
- **Microcard** - Inspiration for multi-app architecture
- **Propel** - Inspiration for prediction market mechanics
- **Akindo** - For hosting the buildathon

## 🔗 Links

- **Live Demo:** https://flash-markets.demo (coming soon)
- **GitHub:** https://github.com/Pratiikpy/Flash--Market
- **Demo Video:** https://www.youtube.com/watch?v=R34zeFm4ArM
- **Discord:** https://discord.gg/linera
- **Linera Docs:** https://linera.dev/

---

**Built with ❤️ on Linera - Don't be late. Be real-time.**

*Submitted for Linera Wave Hack - Wave 3*
