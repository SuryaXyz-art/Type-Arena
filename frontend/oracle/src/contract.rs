#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::OracleState;
use abi::Outcome;
use linera_sdk::{
    linera_base_types::WithContractAbi,
    views::{RootView, View},
    Contract, ContractRuntime,
};
use market::MarketOperation;
use oracle::{OracleMessage, OracleOperation, OracleParameters, OracleResponse, PriceFeed};

pub struct OracleContract {
    state: OracleState,
    runtime: ContractRuntime<Self>,
}

linera_sdk::contract!(OracleContract);

impl WithContractAbi for OracleContract {
    type Abi = oracle::OracleAbi;
}

impl Contract for OracleContract {
    type Message = OracleMessage;
    type Parameters = OracleParameters;
    type InstantiationArgument = ();
    type EventValue = ();

    async fn load(runtime: ContractRuntime<Self>) -> Self {
        let state = OracleState::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        OracleContract { state, runtime }
    }

    async fn instantiate(&mut self, _argument: Self::InstantiationArgument) {
        self.runtime.application_parameters();

        log::info!(
            "Oracle app instantiated on chain {:?}",
            self.runtime.chain_id()
        );
    }

    async fn execute_operation(&mut self, operation: Self::Operation) -> Self::Response {
        match operation {
            OracleOperation::SubmitPrice { symbol, price } => {
                let timestamp = self.runtime.system_time();

                let feed = PriceFeed {
                    symbol: symbol.clone(),
                    price,
                    timestamp,
                };

                self.state
                    .prices
                    .insert(&symbol, feed)
                    .expect("Failed to store price");

                log::info!(
                    "Price feed submitted: {} = {} at {:?}",
                    symbol,
                    price,
                    timestamp
                );

                OracleResponse::Ok
            }

            OracleOperation::ResolveMarketByPrice {
                market_id,
                symbol,
                target_price,
            } => {
                // Get latest price
                let feed = self
                    .state
                    .prices
                    .get(&symbol)
                    .await
                    .expect("Failed to get price")
                    .expect("No price feed for symbol");

                // Determine outcome
                let outcome = if feed.price >= target_price {
                    Outcome::Up
                } else {
                    Outcome::Down
                };

                // Call market app to resolve
                let params = self.runtime.application_parameters();
                self.runtime
                    .call_application(
                        true,
                        params.market_app,
                        &MarketOperation::ResolveMarket { market_id, outcome },
                    );

                log::info!(
                    "Market {} resolved by price: {} (target: {}) = {:?}",
                    market_id,
                    feed.price,
                    target_price,
                    outcome
                );

                OracleResponse::Ok
            }

            OracleOperation::ManualResolve { market_id, outcome } => {
                let params = self.runtime.application_parameters();
                let signer = self.runtime.authenticated_signer();

                if signer != Some(params.owner) {
                    panic!("Only oracle owner can manually resolve markets");
                }

                self.runtime
                    .call_application(
                        true,
                        params.market_app,
                        &MarketOperation::ResolveMarket { market_id, outcome },
                    );

                log::info!("Market {} manually resolved: {:?}", market_id, outcome);

                OracleResponse::Ok
            }
        }
    }

    async fn execute_message(&mut self, message: Self::Message) {
        match message {
            OracleMessage::PriceUpdated {
                symbol,
                price,
                timestamp,
            } => {
                log::info!("Price updated: {} = {} at {:?}", symbol, price, timestamp);
            }

            OracleMessage::ResolveMarketRequest { market_id } => {
                log::info!("Market resolution requested for market {}", market_id);
            }
        }
    }

    async fn store(mut self) {
        self.state.save().await.expect("Failed to save state");
    }
}
