#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::MarketState;
use abi::{
    Bet, Market, MarketStatus, Outcome, Payout, MARKET_CREATION_FEE, MAX_MARKET_DURATION_MICROS,
    MIN_BET_AMOUNT, MIN_MARKET_DURATION_MICROS,
};
use linera_sdk::linera_base_types::{Amount, Timestamp};
use linera_sdk::{
    linera_base_types::WithContractAbi,
    views::{RootView, View},
    Contract, ContractRuntime,
};
use market::{MarketMessage, MarketOperation, MarketParameters, MarketResponse};
use token::{TokenOperation, TokenResponse};

pub struct MarketContract {
    state: MarketState,
    runtime: ContractRuntime<Self>,
}

linera_sdk::contract!(MarketContract);

impl WithContractAbi for MarketContract {
    type Abi = market::MarketAbi;
}

impl Contract for MarketContract {
    type Message = MarketMessage;
    type Parameters = MarketParameters;
    type InstantiationArgument = ();
    type EventValue = ();

    async fn load(runtime: ContractRuntime<Self>) -> Self {
        let state = MarketState::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        MarketContract { state, runtime }
    }

    async fn instantiate(&mut self, _argument: Self::InstantiationArgument) {
        self.runtime.application_parameters();

        log::info!(
            "Market app instantiated on chain {:?}",
            self.runtime.chain_id()
        );
    }

    async fn execute_operation(&mut self, operation: Self::Operation) -> Self::Response {
        match operation {
            MarketOperation::CreateMarket {
                market_type,
                duration_minutes,
            } => {
                let creator = self
                    .runtime
                    .authenticated_signer()
                    .expect("Market creation must be signed");

                let duration_micros = duration_minutes * 60_000_000;

                if duration_micros < MIN_MARKET_DURATION_MICROS
                    || duration_micros > MAX_MARKET_DURATION_MICROS
                {
                    panic!("Invalid market duration");
                }

                let params = self.runtime.application_parameters();
                let balance_response: TokenResponse = self
                    .runtime
                    .call_application(true, params.token_app, &TokenOperation::Balance { owner: creator.to_string() });

                let balance = match balance_response {
                    TokenResponse::Balance(amount) => amount,
                    _ => panic!("Unexpected response"),
                };

                let creation_fee = Amount::from_attos(MARKET_CREATION_FEE);
                if balance < creation_fee {
                    panic!("Insufficient balance for market creation fee");
                }

                let new_balance = balance.saturating_sub(creation_fee);
                self.runtime
                    .call_application(
                        true,
                        params.token_app,
                        &TokenOperation::UpdateBalance {
                            owner: creator.to_string(),
                            amount: new_balance,
                        },
                    );

                let market_id = *self.state.next_market_id.get();
                *self.state.next_market_id.get_mut() += 1;

                let current_time = self.runtime.system_time();
                let closes_at = Timestamp::from(current_time.micros().saturating_add(duration_micros));

                let market = Market {
                    id: market_id,
                    creator,
                    market_type,
                    duration_micros,
                    created_at: current_time,
                    closes_at,
                    status: MarketStatus::Open,
                    total_pool: Amount::ZERO,
                    up_pool: Amount::ZERO,
                    down_pool: Amount::ZERO,
                    outcome: None,
                    resolved_at: None,
                };

                self.state
                    .markets
                    .insert(&market_id, market.clone())
                    .expect("Failed to create market");

                log::info!(
                    "Market {} created by {:?}, closes at {:?}",
                    market_id,
                    creator,
                    closes_at
                );

                MarketResponse::MarketId(market_id)
            }

            MarketOperation::PlaceBet {
                market_id,
                prediction,
                amount,
            } => {
                let bettor = self
                    .runtime
                    .authenticated_signer()
                    .expect("Bet must be signed");

                if amount < Amount::from_attos(MIN_BET_AMOUNT) {
                    panic!("Bet amount too small");
                }

                let mut market = self
                    .state
                    .markets
                    .get(&market_id)
                    .await
                    .expect("Failed to get market")
                    .expect("Market not found");

                let current_time = self.runtime.system_time();
                if !market.is_open(current_time) {
                    panic!("Market is closed");
                }

                let params = self.runtime.application_parameters();
                let balance_response: TokenResponse = self
                    .runtime
                    .call_application(true, params.token_app, &TokenOperation::Balance { owner: bettor.to_string() });

                let balance = match balance_response {
                    TokenResponse::Balance(amount) => amount,
                    _ => panic!("Unexpected response"),
                };

                if balance < amount {
                    panic!("Insufficient balance");
                }

                let new_balance = balance.saturating_sub(amount);
                self.runtime
                    .call_application(
                        true,
                        params.token_app,
                        &TokenOperation::UpdateBalance {
                            owner: bettor.to_string(),
                            amount: new_balance,
                        },
                    );

                let bet = Bet {
                    bettor,
                    amount,
                    prediction,
                    timestamp: current_time,
                };

                let mut bets = self
                    .state
                    .bets
                    .get(&market_id)
                    .await
                    .expect("Failed to get bets")
                    .unwrap_or_default();
                bets.push(bet.clone());
                self.state
                    .bets
                    .insert(&market_id, bets)
                    .expect("Failed to store bets");

                let mut user_bets = self
                    .state
                    .user_bets
                    .get(&(market_id, bettor))
                    .await
                    .expect("Failed to get user bets")
                    .unwrap_or_default();
                user_bets.push(bet);
                self.state
                    .user_bets
                    .insert(&(market_id, bettor), user_bets)
                    .expect("Failed to store user bets");

                market.total_pool = market.total_pool.saturating_add(amount);
                match prediction {
                    abi::Prediction::Up => {
                        market.up_pool = market.up_pool.saturating_add(amount);
                    }
                    abi::Prediction::Down => {
                        market.down_pool = market.down_pool.saturating_add(amount);
                    }
                }

                self.state
                    .markets
                    .insert(&market_id, market)
                    .expect("Failed to update market");

                log::info!("Bet placed: {} on market {} predicting {:?}", amount, market_id, prediction);

                MarketResponse::Ok
            }

            MarketOperation::ResolveMarket { market_id, outcome } => {
                let params = self.runtime.application_parameters();
                let caller = self.runtime.authenticated_caller_id();

                if caller != Some(params.oracle_app_id) {
                    panic!("Only oracle app can resolve markets");
                }

                let mut market = self
                    .state
                    .markets
                    .get(&market_id)
                    .await
                    .expect("Failed to get market")
                    .expect("Market not found");

                let current_time = self.runtime.system_time();

                if !market.can_resolve(current_time) {
                    panic!("Market not ready for resolution");
                }

                market.status = MarketStatus::Resolved;
                market.outcome = Some(outcome);
                market.resolved_at = Some(current_time);

                self.state
                    .markets
                    .insert(&market_id, market)
                    .expect("Failed to resolve market");

                log::info!("Market {} resolved with outcome {:?}", market_id, outcome);

                MarketResponse::Ok
            }

            MarketOperation::CancelMarket { market_id } => {
                let mut market = self
                    .state
                    .markets
                    .get(&market_id)
                    .await
                    .expect("Failed to get market")
                    .expect("Market not found");

                let signer = self
                    .runtime
                    .authenticated_signer()
                    .expect("Cancel must be signed");

                if signer != market.creator {
                    panic!("Only market creator can cancel");
                }

                if market.status != MarketStatus::Open {
                    panic!("Can only cancel open markets");
                }

                market.status = MarketStatus::Cancelled;

                self.state
                    .markets
                    .insert(&market_id, market)
                    .expect("Failed to cancel market");

                let bets = self
                    .state
                    .bets
                    .get(&market_id)
                    .await
                    .expect("Failed to get bets")
                    .unwrap_or_default();

                let params = self.runtime.application_parameters();
                for bet in bets {
                    let balance_response: TokenResponse = self
                        .runtime
                        .call_application(true, params.token_app, &TokenOperation::Balance { owner: bet.bettor.to_string() });

                    let balance = match balance_response {
                        TokenResponse::Balance(amount) => amount,
                        _ => panic!("Unexpected response"),
                    };

                    let new_balance = balance.saturating_add(bet.amount);
                    self.runtime
                        .call_application(
                            true,
                            params.token_app,
                            &TokenOperation::UpdateBalance {
                                owner: bet.bettor.to_string(),
                                amount: new_balance,
                            },
                        );
                }

                log::info!("Market {} cancelled, all bets refunded", market_id);

                MarketResponse::Ok
            }

            MarketOperation::ClaimWinnings { market_id } => {
                let claimer = self
                    .runtime
                    .authenticated_signer()
                    .expect("Claim must be signed");

                let already_claimed = self
                    .state
                    .claimed
                    .get(&(market_id, claimer))
                    .await
                    .expect("Failed to check claim status")
                    .unwrap_or(false);

                if already_claimed {
                    panic!("Winnings already claimed");
                }

                let market = self
                    .state
                    .markets
                    .get(&market_id)
                    .await
                    .expect("Failed to get market")
                    .expect("Market not found");

                if market.status != MarketStatus::Resolved {
                    panic!("Market not resolved");
                }

                let outcome = market.outcome.expect("Resolved market must have outcome");

                let user_bets = self
                    .state
                    .user_bets
                    .get(&(market_id, claimer))
                    .await
                    .expect("Failed to get user bets")
                    .unwrap_or_default();

                if user_bets.is_empty() {
                    panic!("No bets placed");
                }

                let mut total_payout = Amount::ZERO;
                for bet in &user_bets {
                    if bet.prediction == outcome.into() {
                        let payout = market.calculate_payout(bet.amount);
                        total_payout = total_payout.saturating_add(payout);
                    }
                }

                if total_payout == Amount::ZERO {
                    panic!("No winnings to claim");
                }

                self.state
                    .claimed
                    .insert(&(market_id, claimer), true)
                    .expect("Failed to mark as claimed");

                let params = self.runtime.application_parameters();
                let balance_response: TokenResponse = self
                    .runtime
                    .call_application(true, params.token_app, &TokenOperation::Balance { owner: claimer.to_string() });

                let balance = match balance_response {
                    TokenResponse::Balance(amount) => amount,
                    _ => panic!("Unexpected response"),
                };

                let new_balance = balance.saturating_add(total_payout);
                self.runtime
                    .call_application(
                        true,
                        params.token_app,
                        &TokenOperation::UpdateBalance {
                            owner: claimer.to_string(),
                            amount: new_balance,
                        },
                    );

                let payout = Payout {
                    market_id,
                    winner: claimer,
                    amount: total_payout,
                    paid_at: self.runtime.system_time(),
                };

                let payout_id = market_id;
                self.state
                    .payouts
                    .insert(&payout_id, payout)
                    .expect("Failed to record payout");

                log::info!("Paid out {} to {:?} for market {}", total_payout, claimer, market_id);

                MarketResponse::Payout(total_payout)
            }
        }
    }

    async fn execute_message(&mut self, message: Self::Message) {
        match message {
            MarketMessage::MarketCreated { market_id, creator } => {
                log::info!("Market {} created by {:?}", market_id, creator);
            }

            MarketMessage::MarketResolved { market_id, outcome } => {
                log::info!("Market {} resolved with outcome {:?}", market_id, outcome);
            }

            MarketMessage::BetPlaced {
                market_id,
                bettor,
                prediction,
                amount,
            } => {
                log::info!(
                    "Bet placed: {} on market {} by {:?} predicting {:?}",
                    amount,
                    market_id,
                    bettor,
                    prediction
                );
            }
        }
    }

    async fn store(mut self) {
        self.state.save().await.expect("Failed to save state");
    }
}
