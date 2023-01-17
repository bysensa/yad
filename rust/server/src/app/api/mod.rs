use jsonrpsee::{proc_macros::rpc, core::async_trait, core::RpcResult};

use super::AppState;


#[rpc(server, namespace = "state")]
pub trait CommonRpc<T> {
    #[method(name = "health_check")]
    async fn healthcheck(&self) -> RpcResult<i32>;
}


pub struct AppRpcModule;

#[async_trait]
impl CommonRpcServer<AppState> for AppRpcModule {
    async fn healthcheck(&self) -> RpcResult<i32> {
        Ok(0)
    }
}