#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use self::state::TokenState;
use token::{DailyBonus, TokenMessage, TokenOperation, TokenParameters, TokenResponse};
use linera_sdk::linera_base_types::{AccountOwner, Amount};
use linera_sdk::{
    linera_base_types::WithContractAbi,
    views::{RootView, View},
    Contract, ContractRuntime,
};

pub struct TokenContract {
    state: TokenState,
    runtime: ContractRuntime<Self>,
}

linera_sdk::contract!(TokenContract);

impl WithContractAbi for TokenContract {
    type Abi = token::TokenAbi;
}

impl Contract for TokenContract {
    type Message = TokenMessage;
    type Parameters = TokenParameters;
    type InstantiationArgument = ();
    type EventValue = ();

    async fn load(runtime: ContractRuntime<Self>) -> Self {
        let state = TokenState::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        TokenContract { state, runtime }
    }

    async fn instantiate(&mut self, _argument: Self::InstantiationArgument) {
        let params = self.runtime.application_parameters();

        // Set initial total supply
        self.state.total_supply.set(params.initial_supply);

        log::info!(
            "Token app instantiated on chain {:?} with supply {}",
            self.runtime.chain_id(),
            params.initial_supply
        );
    }

    async fn execute_operation(&mut self, operation: Self::Operation) -> Self::Response {
        match operation {
            TokenOperation::Balance { owner } => {
                let owner: AccountOwner = owner.parse().expect("Invalid owner");
                log::info!("Getting balance for {:?}", owner);

                let balance = self
                    .state
                    .accounts
                    .get(&owner)
                    .await
                    .expect("Failed to get balance")
                    .unwrap_or(Amount::ZERO);

                TokenResponse::Balance(balance)
            }

            TokenOperation::UpdateBalance { owner, amount } => {
                let owner: AccountOwner = owner.parse().expect("Invalid owner");
                log::info!("Updating balance for {:?} to {}", owner, amount);

                self.state
                    .accounts
                    .insert(&owner, amount)
                    .expect("Failed to update balance");

                TokenResponse::Ok
            }

            TokenOperation::Transfer { from, to, amount } => {
                let from: AccountOwner = from.parse().expect("Invalid from address");
                let to: AccountOwner = to.parse().expect("Invalid to address");
                log::info!("Transferring {} from {:?} to {:?}", amount, from, to);

                // Verify signer
                let signer = self
                    .runtime
                    .authenticated_signer()
                    .expect("Transfer must be signed");

                if signer != from {
                    panic!("Only account owner can transfer");
                }

                // Get balances
                let from_balance = self
                    .state
                    .accounts
                    .get(&from)
                    .await
                    .expect("Failed to get sender balance")
                    .unwrap_or(Amount::ZERO);

                if from_balance < amount {
                    panic!("Insufficient balance");
                }

                let to_balance = self
                    .state
                    .accounts
                    .get(&to)
                    .await
                    .expect("Failed to get receiver balance")
                    .unwrap_or(Amount::ZERO);

                // Update balances
                self.state
                    .accounts
                    .insert(&from, from_balance.saturating_sub(amount))
                    .expect("Failed to update sender balance");

                self.state
                    .accounts
                    .insert(&to, to_balance.saturating_add(amount))
                    .expect("Failed to update receiver balance");

                log::info!("Transfer completed");
                TokenResponse::Ok
            }

            TokenOperation::Mint { to, amount } => {
                let to: AccountOwner = to.parse().expect("Invalid to address");
                log::info!("Minting {} tokens to {:?}", amount, to);

                // Only master chain can mint
                let params = self.runtime.application_parameters();
                if self.runtime.chain_id() != params.master_chain {
                    panic!("Only master chain can mint tokens");
                }

                // Update balance
                let current_balance = self
                    .state
                    .accounts
                    .get(&to)
                    .await
                    .expect("Failed to get balance")
                    .unwrap_or(Amount::ZERO);

                self.state
                    .accounts
                    .insert(&to, current_balance.saturating_add(amount))
                    .expect("Failed to update balance");

                // Update total supply
                let total = self.state.total_supply.get_mut();
                *total = total.saturating_add(amount);

                log::info!("Minted {} tokens, new total supply: {}", amount, *total);
                TokenResponse::Ok
            }

            TokenOperation::ClaimBonus { owner } => {
                let owner: AccountOwner = owner.parse().expect("Invalid owner");
                log::info!("Claiming daily bonus for {:?}", owner);

                let params = self.runtime.application_parameters();
                let current_time = self.runtime.system_time().micros();

                // Get or create daily bonus record
                let mut bonus = self
                    .state
                    .daily_bonuses
                    .get(&owner)
                    .await
                    .expect("Failed to get daily bonus")
                    .unwrap_or(DailyBonus {
                        amount: params.daily_bonus,
                        last_claim: 0,
                    });

                // Attempt to claim
                let claimed_amount = bonus.claim(current_time);

                if claimed_amount == Amount::ZERO {
                    panic!("Daily bonus not available yet");
                }

                // Update bonus record
                self.state
                    .daily_bonuses
                    .insert(&owner, bonus)
                    .expect("Failed to update daily bonus");

                // Update balance
                let current_balance = self
                    .state
                    .accounts
                    .get(&owner)
                    .await
                    .expect("Failed to get balance")
                    .unwrap_or(Amount::ZERO);

                self.state
                    .accounts
                    .insert(&owner, current_balance.saturating_add(claimed_amount))
                    .expect("Failed to update balance");

                log::info!("Claimed {} tokens", claimed_amount);
                TokenResponse::Balance(current_balance.saturating_add(claimed_amount))
            }
        }
    }

    async fn execute_message(&mut self, message: Self::Message) {
        match message {
            TokenMessage::BalanceUpdated { owner, new_balance } => {
                log::info!("Received balance update for {:?}: {}", owner, new_balance);

                self.state
                    .accounts
                    .insert(&owner, new_balance)
                    .expect("Failed to update balance from message");
            }

            TokenMessage::TokensMinted { to, amount } => {
                log::info!("Received tokens minted notification: {} to {:?}", amount, to);
            }
        }
    }

    async fn store(mut self) {
        self.state.save().await.expect("Failed to save state");
    }
}
