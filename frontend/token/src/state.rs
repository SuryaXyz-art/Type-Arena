use token::DailyBonus;
use linera_sdk::linera_base_types::{AccountOwner, Amount};
use linera_sdk::views::{linera_views, MapView, RegisterView, RootView, ViewStorageContext};

#[derive(RootView)]
#[view(context = ViewStorageContext)]
pub struct TokenState {
    /// User account balances
    pub accounts: MapView<AccountOwner, Amount>,

    /// Daily bonus state per user
    pub daily_bonuses: MapView<AccountOwner, DailyBonus>,

    /// Total supply
    pub total_supply: RegisterView<Amount>,
}
