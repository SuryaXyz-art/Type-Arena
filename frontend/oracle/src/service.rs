#![cfg_attr(target_arch = "wasm32", no_main)]

mod state;

use async_graphql::{EmptySubscription, Object, Request, Response, Schema};
use linera_sdk::{
    abi::WithServiceAbi,
    graphql::GraphQLMutationRoot,
    views::{MapView, View},
    Service, ServiceRuntime,
};
use oracle::{OracleOperation, PriceFeed};
use std::sync::Arc;
use self::state::OracleState;

#[derive(Clone)]
pub struct OracleService {
    state: Arc<OracleState>,
    runtime: Arc<ServiceRuntime<Self>>,
}

linera_sdk::service!(OracleService);

impl WithServiceAbi for OracleService {
    type Abi = oracle::OracleAbi;
}

impl Service for OracleService {
    type Parameters = ();

    async fn new(runtime: ServiceRuntime<Self>) -> Self {
        let state = OracleState::load(runtime.root_view_storage_context())
            .await
            .expect("Failed to load state");
        OracleService {
            state: Arc::new(state),
            runtime: Arc::new(runtime),
        }
    }

    async fn handle_query(&self, request: Request) -> Response {
        let schema = Schema::build(
            self.clone(),
            OracleOperation::mutation_root(self.runtime.clone()),
            EmptySubscription,
        )
        .finish();

        schema.execute(request).await
    }
}

#[Object]
impl OracleService {
    async fn prices(&self) -> &MapView<String, PriceFeed> {
        &self.state.prices
    }
}
