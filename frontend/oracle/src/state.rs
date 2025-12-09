use oracle::PriceFeed;
use linera_sdk::views::{linera_views, MapView, RootView, ViewStorageContext};

#[derive(RootView)]
#[view(context = ViewStorageContext)]
pub struct OracleState {
    /// Latest price feeds per symbol
    pub prices: MapView<String, PriceFeed>,
}
