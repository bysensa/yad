use async_trait::async_trait;
use jsonrpsee::core::*;
use jsonrpsee::proc_macros::rpc;
use jsonrpsee::RpcModule;
use salvo::prelude::*;
use std::sync::Arc;
use yad_net::rpc::RpcReply;

#[cfg(test)]
mod tests {
    use crate::net::router;
    use salvo::prelude::*;

    #[async_std::test]
    async fn handle_rpc_test() {
        let addr = "127.0.0.1:7878";
        let route = router();
        async_std::task::spawn(Server::new(TcpListener::bind(addr)).serve(route));

        let client = reqwest::Client::new();
        let res = client
            .post("http://127.0.0.1:7878/tracking")
            .body(r#"{"jsonrpc": "2.0", "method": "tracking_bar", "params": [42, 23], "id": 1}"#)
            .send()
            .await
            .unwrap()
            .text()
            .await;
        dbg!(&res);
    }
}

#[handler]
async fn rpc_handler(req: &mut Request, res: &mut Response, depot: &mut Depot) {
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
pub struct YadTrackingRpcContext;

impl YadTrackingRpcContext {
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
