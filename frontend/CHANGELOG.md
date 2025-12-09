# Changelog

All notable changes to Flash Markets will be documented in this file.

## [Wave 3] - 2025-12-01

### 🎉 Initial Release

#### Added
- **Three-application architecture**
  - Token App: Balance management, daily bonuses, transfers
  - Market App: Create markets, place bets, resolve, claim winnings
  - Oracle App: Price feeds, automated resolution

- **Core Features**
  - Ultra-fast prediction markets (1-60 minute resolution)
  - Real-time bet placement with instant confirmation
  - Automated market resolution via oracle
  - Fair algorithmic payout distribution (95% to winners)
  - Platform fee system (5%)
  - Market cancellation with refunds

- **Token Economics**
  - Daily bonus system (24-hour cooldown)
  - Transfer operations
  - Cross-application balance management
  - Mint operations (master chain only)

- **Market Types**
  - Price prediction markets (BTC, ETH, etc.)
  - Binary event markets (Yes/No questions)
  - Custom markets with manual resolution

- **Developer Experience**
  - Comprehensive GraphQL API
  - Docker deployment setup
  - Build scripts (Makefile, build.sh)
  - Full documentation
  - Type-safe Rust implementation

#### Technical Highlights
- Cross-application calls (Market ↔ Token, Oracle → Market)
- Multi-chain architecture leveraging Linera microchains
- Real-time state updates
- Event-driven resolution system
- Secure authorization checks

#### Constants & Configuration
- Min bet: 0.1 tokens
- Market creation fee: 1 token
- Platform fee: 5%
- Min market duration: 1 minute
- Max market duration: 24 hours
- Daily bonus cooldown: 24 hours

### 📊 Statistics
- **Lines of Code:** ~2,500
- **Applications:** 3 (Token, Market, Oracle)
- **State Structures:** 3 (MapView, RegisterView, CollectionView)
- **Operations:** 13 total across all apps
- **GraphQL Queries:** 15+
- **Test Coverage:** Ready for integration tests

### 🎯 Buildathon Requirements Met

✅ **Working Demo** - All applications compile and integrate
✅ **Linera Stack Integration** - Multi-chain, cross-app calls, microchains
✅ **Creativity & UX** - Fast markets showcasing real-time capabilities
✅ **Scalability** - Easy to add new market types
✅ **Vision** - Clear roadmap for Waves 4-6

### 🚀 Deployment
- Docker Compose configuration
- Conway Testnet compatible
- One-command deployment
- GraphQL service on port 8080

### 📝 Documentation
- Comprehensive README (15KB)
- Code comments and documentation
- GraphQL API examples
- Architecture diagrams
- Deployment instructions

## [Wave 4] - Planned

### Planned Features
- React frontend with real-time updates
- Event streaming for live market data
- Market analytics dashboard
- Mobile-responsive UI
- Historical data tracking

## [Wave 5] - Planned

### Planned Features
- AI-powered market resolution (agentic bonus)
- Additional market types (sports, gaming, events)
- Liquidity pools for market making
- Referral system
- Advanced user profiles

## [Wave 6] - Planned

### Planned Features
- Cross-chain oracle integration
- Advanced charting and analytics
- Social features (following, leaderboards)
- Tournament mode
- Governance system

---

**Note:** This changelog follows the Linera WaveHack submission schedule.
Each wave represents a major iteration with new features and improvements.
