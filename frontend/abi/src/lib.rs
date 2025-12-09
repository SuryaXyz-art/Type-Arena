use async_graphql::{scalar, SimpleObject};
use linera_sdk::linera_base_types::{AccountOwner, Amount, Timestamp};
use serde::{Deserialize, Serialize};

// ============================================================================
// SHARED TYPES
// ============================================================================

/// Market prediction type
#[derive(Debug, Clone, Copy, Deserialize, Eq, PartialEq, Serialize)]
#[repr(u8)]
pub enum Prediction {
    Up = 0,    // Price will go up / Yes
    Down = 1,  // Price will go down / No
}

scalar!(Prediction);

/// Market status
#[derive(Debug, Clone, Copy, Deserialize, Eq, PartialEq, Serialize)]
#[repr(u8)]
pub enum MarketStatus {
    Open = 0,      // Accepting bets
    Locked = 1,    // No more bets, waiting for resolution
    Resolved = 2,  // Market resolved with outcome
    Cancelled = 3, // Market cancelled, refunds issued
}

scalar!(MarketStatus);

/// Market outcome (what actually happened)
#[derive(Debug, Clone, Copy, Deserialize, Eq, PartialEq, Serialize)]
#[repr(u8)]
pub enum Outcome {
    Up = 0,   // Price went up / Yes
    Down = 1, // Price went down / No
}

scalar!(Outcome);

/// Market type
#[derive(Debug, Clone, Deserialize, Eq, PartialEq, Serialize)]
pub enum MarketType {
    /// Price prediction: Will BTC be higher in X minutes?
    PricePrediction {
        symbol: String,
        target_price: u64,
    },
    /// Binary event: Will X happen?
    BinaryEvent {
        question: String,
    },
    /// Custom market with manual resolution
    Custom {
        description: String,
    },
}

scalar!(MarketType);

/// Bet record
#[derive(Debug, Clone, Deserialize, Eq, PartialEq, Serialize, SimpleObject)]
pub struct Bet {
    pub bettor: AccountOwner,
    pub amount: Amount,
    pub prediction: Prediction,
    pub timestamp: Timestamp,
}

/// Market information
#[derive(Debug, Clone, Deserialize, Eq, PartialEq, Serialize, SimpleObject)]
pub struct Market {
    pub id: u64,
    pub creator: AccountOwner,
    pub market_type: MarketType,
    pub duration_micros: u64,
    pub created_at: Timestamp,
    pub closes_at: Timestamp,
    pub status: MarketStatus,
    pub total_pool: Amount,
    pub up_pool: Amount,
    pub down_pool: Amount,
    pub outcome: Option<Outcome>,
    pub resolved_at: Option<Timestamp>,
}

/// Payout record
#[derive(Debug, Clone, Deserialize, Eq, PartialEq, Serialize, SimpleObject)]
pub struct Payout {
    pub market_id: u64,
    pub winner: AccountOwner,
    pub amount: Amount,
    pub paid_at: Timestamp,
}

// ============================================================================
// CONSTANTS
// ============================================================================

/// Minimum bet amount (0.1 tokens)
pub const MIN_BET_AMOUNT: u128 = 100_000;

/// Market creation fee (1 token)
pub const MARKET_CREATION_FEE: u128 = 1_000_000;

/// Platform fee percentage (5%)
pub const PLATFORM_FEE_PERCENT: u8 = 5;

/// Minimum market duration (1 minute)
pub const MIN_MARKET_DURATION_MICROS: u64 = 60_000_000;

/// Maximum market duration (24 hours)
pub const MAX_MARKET_DURATION_MICROS: u64 = 86_400_000_000;

// ============================================================================
// UTILITY FUNCTIONS
// ============================================================================

impl Market {
    /// Check if market is still accepting bets
    pub fn is_open(&self, current_time: Timestamp) -> bool {
        self.status == MarketStatus::Open && current_time < self.closes_at
    }

    /// Check if market is ready for resolution
    pub fn can_resolve(&self, current_time: Timestamp) -> bool {
        self.status == MarketStatus::Open && current_time >= self.closes_at
    }

    /// Calculate payout for a winning bet
    pub fn calculate_payout(&self, bet_amount: Amount) -> Amount {
        if self.total_pool == Amount::ZERO {
            return Amount::ZERO;
        }

        let winning_pool = match self.outcome {
            Some(Outcome::Up) => self.up_pool,
            Some(Outcome::Down) => self.down_pool,
            None => return Amount::ZERO,
        };

        if winning_pool == Amount::ZERO {
            return Amount::ZERO;
        }

        // Convert Amount to u128 for math operations
        let total_pool_u128: u128 = self.total_pool.into();
        let bet_amount_u128: u128 = bet_amount.into();
        let winning_pool_u128: u128 = winning_pool.into();

        // Payout = (bet_amount / winning_pool) * total_pool * 0.95
        let total_after_fee = total_pool_u128.saturating_mul(95).saturating_div(100);
        let payout = bet_amount_u128
            .saturating_mul(total_after_fee)
            .saturating_div(winning_pool_u128);

        // Convert back to Amount
        Amount::from_tokens(payout)
    }
}

impl Prediction {
    pub fn opposite(&self) -> Self {
        match self {
            Prediction::Up => Prediction::Down,
            Prediction::Down => Prediction::Up,
        }
    }
}

impl From<Outcome> for Prediction {
    fn from(outcome: Outcome) -> Self {
        match outcome {
            Outcome::Up => Prediction::Up,
            Outcome::Down => Prediction::Down,
        }
    }
}

impl From<Prediction> for Outcome {
    fn from(prediction: Prediction) -> Self {
        match prediction {
            Prediction::Up => Outcome::Up,
            Prediction::Down => Outcome::Down,
        }
    }
}
