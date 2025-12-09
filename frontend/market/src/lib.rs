use abi::{Market, MarketType, Outcome, Prediction};
use async_graphql::{Request, Response};
use linera_sdk::linera_base_types::{AccountOwner, Amount, ApplicationId, Timestamp};
use linera_sdk::{
    graphql::GraphQLMutationRoot,
    linera_base_types::{ContractAbi, ServiceAbi},
};
use serde::{Deserialize, Serialize};
use token::TokenAbi;

#[derive(Debug, Deserialize, Serialize)]
pub struct MarketAbi;

impl ContractAbi for MarketAbi {
    type Operation = MarketOperation;
    type Response = MarketResponse;
}

impl ServiceAbi for MarketAbi {
    type Query = Request;
    type QueryResponse = Response;
}

#[derive(Debug, Deserialize, Serialize, GraphQLMutationRoot)]
pub enum MarketOperation {
    /// Create a new market
    CreateMarket {
        market_type: MarketType,
        duration_minutes: u64,
    },

    /// Place a bet on a market
    PlaceBet {
        market_id: u64,
        prediction: Prediction,
        amount: Amount,
    },

    /// Resolve a market (called by oracle or admin)
    ResolveMarket {
        market_id: u64,
        outcome: Outcome,
    },

    /// Cancel a market (admin only, refunds all bets)
    CancelMarket {
        market_id: u64,
    },

    /// Claim winnings from a resolved market
    ClaimWinnings {
        market_id: u64,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum MarketMessage {
    /// Market created notification
    MarketCreated {
        market_id: u64,
        creator: AccountOwner,
    },

    /// Market resolved notification
    MarketResolved {
        market_id: u64,
        outcome: Outcome,
    },

    /// Bet placed notification
    BetPlaced {
        market_id: u64,
        bettor: AccountOwner,
        prediction: Prediction,
        amount: Amount,
    },
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub enum MarketResponse {
    #[default]
    Ok,
    MarketId(u64),
    Market(Market),
    Payout(Amount),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MarketParameters {
    /// Token application ID for handling bets
    pub token_app: ApplicationId<TokenAbi>,

    /// Platform fee wallet
    pub platform_wallet: AccountOwner,

    /// Oracle application ID for resolving markets
    pub oracle_app_id: ApplicationId,
}
