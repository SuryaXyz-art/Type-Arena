#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use async_graphql::{EmptySubscription, Object, Request, Response, Schema};
use linera_sdk::{
    linera_base_types::{AccountOwner, WithServiceAbi},
    graphql::GraphQLMutationRoot,
    views::{MapView, View},
    Service, ServiceRuntime,
};
use market::MarketOperation;
use self::state::MarketState;
use std::sync::Arc;
use abi::{Bet, Market, Payout};

#[derive(Clone)]
pub struct MarketService {
    state: Arc<MarketState>,
    runtime: Arc<ServiceRuntime<Self>>,
}

linera_sdk::service!(MarketService);

impl WithServiceAbi for MarketService {
    type Abi = market::MarketAbi;
}

impl Service for MarketService {
    type Parameters = market::MarketParameters;

    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        let state = MarketState::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        MarketService {
            state: Arc::new(state),
            runtime: Arc::new(runtime),
        }
    }

    async fn handle_query(&self, request: Request) -> Response {
        let schema = Schema::build(
            self.clone(),
            MarketOperation::mutation_root(self.runtime.clone()),
            EmptySubscription,
        )
        .finish();

        schema.execute(request).await
    }
}

#[Object]
impl MarketService {
    async fn next_market_id(&self) -> &u64 {
        self.state.next_market_id.get()
    }

    async fn markets(&self) -> &MapView<u64, Market> {
        &self.state.markets
    }

    async fn bets(&self) -> &MapView<u64, Vec<Bet>> {
        &self.state.bets
    }

    async fn payouts(&self) -> &MapView<u64, Payout> {
        &self.state.payouts
    }
}
