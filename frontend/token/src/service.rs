#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use async_graphql::{EmptySubscription, Object, Request, Response, Schema};
use linera_sdk::{
    abi::WithServiceAbi,
    graphql::GraphQLMutationRoot,
    linera_base_types::{AccountOwner, Amount},
    views::{MapView, View},
    Service, ServiceRuntime,
};
use std::sync::Arc;
use token::{DailyBonus, TokenOperation};
use self::state::TokenState;

#[derive(Clone)]
pub struct TokenService {
    state: Arc<TokenState>,
    runtime: Arc<ServiceRuntime<Self>>,
}

linera_sdk::service!(TokenService);

impl WithServiceAbi for TokenService {
    type Abi = token::TokenAbi;
}

impl Service for TokenService {
    type Parameters = token::TokenParameters;

    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        let state = TokenState::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        TokenService {
            state: Arc::new(state),
            runtime: Arc::new(runtime),
        }
    }

    async fn handle_query(&self, request: Request) -> Response {
        let schema = Schema::build(
            self.clone(),
            TokenOperation::mutation_root(self.runtime.clone()),
            EmptySubscription,
        )
        .finish();

        schema.execute(request).await
    }
}

#[Object]
impl TokenService {
    async fn accounts(&self) -> &MapView<AccountOwner, Amount> {
        &self.state.accounts
    }

    async fn daily_bonuses(&self) -> &MapView<AccountOwner, DailyBonus> {
        &self.state.daily_bonuses
    }

    async fn total_supply(&self) -> &Amount {
        self.state.total_supply.get()
    }
}
