use async_trait::async_trait;
use entrait::Impl;
use jsonrpsee::core::*;
use jsonrpsee::proc_macros::rpc;
use jsonrpsee::RpcModule;
use salvo::prelude::*;
use std::sync::Arc;
use yad_net::rpc::RpcReply;

use crate::ScopeState;


#[handler]
pub async fn rpc_handler(req: &mut Request, res: &mut Response, depot: &mut Depot) {
    let module = depot.obtain::<Arc<YadTrackingRpcModule>>();
    yad_net::rpc::handler(module, req, res).await;
}

pub type YadTrackingRpcModule = RpcModule<YadTrackingRpcContext>;

impl RpcReply for YadTrackingReply {
    type Output = Self;
}

#[derive(Serialize)]
pub enum YadTrackingReply {
    Processed,
}

#[rpc(server, namespace = "tracking",  server_bounds(T::Output: jsonrpsee::core::Serialize))]
pub trait YadTrackingRpc<T: RpcReply> {
    #[method(name = "bar")]
    async fn bar(&self) -> RpcResult<T::Output>;

    #[method(name = "foo")]
    fn foo(&self) -> RpcResult<()>;
}

#[derive(Debug)]
pub struct YadTrackingRpcContext {
    scope: Arc<Impl<ScopeState>>
}

impl YadTrackingRpcContext {
    pub fn new(scope: Arc<Impl<ScopeState>>) -> Self {
        YadTrackingRpcContext {scope} 
    }

    pub fn module(self) -> YadTrackingRpcModule {
        self.into_rpc()
    }
}

#[async_trait]
impl YadTrackingRpcServer<YadTrackingReply> for YadTrackingRpcContext {
    async fn bar(&self) -> Result<<YadTrackingReply as RpcReply>::Output, Error> {
        Ok(YadTrackingReply::Processed)
    }

    fn foo(&self) -> RpcResult<()> {
        Ok(())
    }
}
