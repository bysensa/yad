use salvo::prelude::*;
use std::sync::Arc;

mod processor;
mod rpc;

pub struct Scope {}

impl Scope {
    pub fn new() -> Self {
        Scope {}
    }

    pub fn router() -> Router {
        let rpc_context = rpc::YadTrackingRpcContext;
        let rpc_module = Arc::new(rpc_context.module());
        Router::with_path("tracking")
            .hoop(affix::inject(rpc_module))
            .post(rpc_handler)
    }
}
