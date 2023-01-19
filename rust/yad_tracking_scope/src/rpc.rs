use async_trait::async_trait;
use jsonrpsee::core::*;
use jsonrpsee::proc_macros::rpc;
use jsonrpsee::RpcModule;

#[derive(Serialize)]
pub enum YadTrackingReply {
    Processed,
}

pub trait Reply {
    type Output: Send + Sync + 'static;
}

impl Reply for YadTrackingReply {
    type Output = Self;
}

#[rpc(server, namespace = "tracking",  server_bounds(T::Output: jsonrpsee::core::Serialize))]
pub trait Rpc<T: Reply> {
    #[method(name = "bar")]
    fn method(&self) -> Result<T::Output, Error>;
}

#[derive(Debug)]
pub struct YadTrackingRpcServer;
pub type YadTrackingRpcModule = RpcModule<YadTrackingRpcServer>;

impl YadTrackingRpcServer {
    pub fn module(self) -> YadTrackingRpcModule {
        self.into_rpc()
    }
}

#[async_trait]
impl RpcServer<YadTrackingReply> for YadTrackingRpcServer {
    fn method(&self) -> Result<<YadTrackingReply as Reply>::Output, Error> {
        dbg!(&self);
        Ok(YadTrackingReply::Processed)
    }
}

#[cfg(test)]
mod tests {
    use super::RpcServer;
    use crate::rpc::YadTrackingRpcServer;
    use jsonrpsee::server::ServerBuilder;

    #[async_std::test]
    async fn test() {
        let module = YadTrackingRpcServer.into_rpc();
        let res = module.raw_json_request("{}").await;
        dbg!(res);
    }
}
