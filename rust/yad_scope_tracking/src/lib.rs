use entrait::Impl;
use salvo::prelude::*;
use state::ScopeState;
use std::sync::Arc;
use yad_storage::Database;

mod rpc;
mod state;
mod operations;
mod model;

pub struct Scope {
    state: Arc<Impl<ScopeState>>
}

impl Scope {
    pub fn new(db: Database) -> Self {
        let state = ScopeState::new(db);
        let state = Arc::new(Impl::new(state));
        Scope {state}
    }

    pub fn from_state(state: ScopeState) -> Self {
        Scope { state: Arc::new(Impl::new(state)) }
    }

    pub fn router(&self) -> Router {
        let scope = self.state.clone();
        let rpc_context = rpc::YadTrackingRpcContext::new(scope);
        let rpc_module = Arc::new(rpc_context.module());
        Router::with_path("tracking")
            .hoop(affix::inject(rpc_module))
            .post(rpc::rpc_handler)
    }
}
