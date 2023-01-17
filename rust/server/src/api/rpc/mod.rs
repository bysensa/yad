mod activity;

use entrait::Impl;
use jsonrpsee::core::server::rpc_module::RpcModule;
use salvo::{handler, prelude::StatusCode, Depot, Request, Response, Router};
use serde::Serialize;
use std::str::from_utf8;

use crate::app::{App, api::{AppRpcModule, CommonRpcServer}};

pub(crate) type Context = Impl<App>;
pub type RpcApi = RpcModule<Impl<App>>;

pub fn router() -> Router {
    Router::with_path("rpc").hoop(set_rpc).post(rpc_handler)
}

#[handler]
async fn set_rpc(depot: &mut Depot) {
    let app = depot.obtain::<Impl<App>>().unwrap().clone();
    let rpc_module = get_rpc_module(app).unwrap();
    depot.inject::<RpcApi>(rpc_module);
}

#[handler]
async fn rpc_handler(depot: &mut Depot, req: &mut Request, res: &mut Response) {
    let Some(module) = depot.obtain::<RpcApi>() else {
        return res.set_status_code(StatusCode::INTERNAL_SERVER_ERROR);
    };
    let Ok(msg) = req.payload().await else {
        return res.set_status_code(StatusCode::INTERNAL_SERVER_ERROR);
    };
    let msg = msg.as_slice();
    let Ok(msg) = from_utf8(msg) else {
        return res.set_status_code(StatusCode::INTERNAL_SERVER_ERROR);
    };
    let Ok((result, mut _stream)) = module.raw_json_request(msg).await else {
        return res.set_status_code(StatusCode::INTERNAL_SERVER_ERROR);
    };

    let result = result.result;
    res.set_status_code(StatusCode::OK);
    res.render(result);
}

fn get_rpc_module(app: Impl<App>) -> anyhow::Result<RpcApi> {
    let mut module = AppRpcModule.into_rpc();
    register_healthcheck(&mut module)?;
    activity::register_methods(&mut module)?;
    Ok(module)
}

fn register_healthcheck(module: &mut RpcApi) -> anyhow::Result<()> {
    module.register_async_method("healthcheck", |params, ctx| async move {
        Ok(Health { is_health: true })
    })?;
    Ok(())
}

#[derive(Serialize)]
struct Health {
    is_health: bool,
}
