use abi::Outcome;
use async_graphql::{Request, Response, SimpleObject};
use linera_sdk::linera_base_types::{ApplicationId, Timestamp};
use linera_sdk::{
    graphql::GraphQLMutationRoot,
    linera_base_types::{ContractAbi, ServiceAbi},
};
use market::MarketAbi;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct OracleAbi;

impl ContractAbi for OracleAbi {
    type Operation = OracleOperation;
    type Response = OracleResponse;
}

impl ServiceAbi for OracleAbi {
    type Query = Request;
    type QueryResponse = Response;
}

#[derive(Debug, Deserialize, Serialize, GraphQLMutationRoot)]
pub enum OracleOperation {
    /// Submit a price feed
    SubmitPrice { symbol: String, price: u64 },

    /// Resolve a market based on price
    ResolveMarketByPrice {
        market_id: u64,
        symbol: String,
        target_price: u64,
    },

    /// Manually resolve a market (admin only)
    ManualResolve { market_id: u64, outcome: Outcome },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum OracleMessage {
    /// Price update notification
    PriceUpdated { symbol: String, price: u64, timestamp: Timestamp },

    /// Market resolution request
    ResolveMarketRequest { market_id: u64 },
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub enum OracleResponse {
    #[default]
    Ok,
    Price(u64),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OracleParameters {
    /// Market application ID
    pub market_app: ApplicationId<MarketAbi>,

    /// Oracle owner (admin)
    pub owner: linera_sdk::linera_base_types::AccountOwner,
}

#[derive(Debug, Clone, Deserialize, Eq, PartialEq, Serialize, SimpleObject)]
pub struct PriceFeed {
    pub symbol: String,
    pub price: u64,
    pub timestamp: Timestamp,
}
