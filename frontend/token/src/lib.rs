use async_graphql::{Request, Response, SimpleObject};
use linera_sdk::linera_base_types::{AccountOwner, Amount, ChainId};
use linera_sdk::{
    graphql::GraphQLMutationRoot,
    linera_base_types::{ContractAbi, ServiceAbi},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenAbi;

impl ContractAbi for TokenAbi {
    type Operation = TokenOperation;
    type Response = TokenResponse;
}

impl ServiceAbi for TokenAbi {
    type Query = Request;
    type QueryResponse = Response;
}

#[derive(Debug, Deserialize, Serialize, GraphQLMutationRoot)]
pub enum TokenOperation {
    /// Get account balance
    Balance { owner: String },

    /// Update account balance (called by market app)
    UpdateBalance { owner: String, amount: Amount },

    /// Transfer tokens between accounts
    Transfer {
        from: String,
        to: String,
        amount: Amount,
    },

    /// Mint tokens (master chain only)
    Mint { to: String, amount: Amount },

    /// Claim daily bonus
    ClaimBonus { owner: String },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum TokenMessage {
    /// Balance updated notification
    BalanceUpdated { owner: AccountOwner, new_balance: Amount },

    /// Tokens minted notification
    TokensMinted { to: AccountOwner, amount: Amount },
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub enum TokenResponse {
    #[default]
    Ok,
    Balance(Amount),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TokenParameters {
    pub master_chain: ChainId,
    pub initial_supply: Amount,
    pub daily_bonus: Amount,
}

#[derive(Debug, Clone, Default, Deserialize, Eq, PartialEq, Serialize, SimpleObject)]
pub struct DailyBonus {
    pub amount: Amount,
    pub last_claim: u64, // Timestamp in micros
}

impl DailyBonus {
    pub const ONE_DAY_MICROS: u64 = 86_400_000_000;

    pub fn can_claim(&self, current_time_micros: u64) -> bool {
        if self.last_claim == 0 {
            return true;
        }
        current_time_micros.saturating_sub(self.last_claim) >= Self::ONE_DAY_MICROS
    }

    pub fn claim(&mut self, current_time_micros: u64) -> Amount {
        if self.can_claim(current_time_micros) {
            self.last_claim = current_time_micros;
            self.amount
        } else {
            Amount::ZERO
        }
    }
}
