use abi::{Bet, Market, Payout};
use linera_sdk::linera_base_types::AccountOwner;
use linera_sdk::views::{linera_views, MapView, RegisterView, RootView, ViewStorageContext};

/// Market application state
#[derive(RootView)]
#[view(context = ViewStorageContext)]
pub struct MarketState {
    /// Next market ID
    pub next_market_id: RegisterView<u64>,

    /// Active markets (market_id -> Market)
    pub markets: MapView<u64, Market>,

    /// Bets per market (market_id -> Vec<Bet>)
    pub bets: MapView<u64, Vec<Bet>>,

    /// User bets per market (market_id -> AccountOwner -> Vec<Bet>)
    pub user_bets: MapView<(u64, AccountOwner), Vec<Bet>>,

    /// Claimed payouts (market_id -> AccountOwner -> bool)
    pub claimed: MapView<(u64, AccountOwner), bool>,

    /// Payout records (key: payout_id, value: Payout)
    pub payouts: MapView<u64, Payout>,
}
