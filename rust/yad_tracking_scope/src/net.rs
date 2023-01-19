use crate::rpc::YadTrackingRpcModule;
use salvo::http::ParseError;
use salvo::prelude::*;
use std::str::from_utf8;
use std::sync::Arc;

pub fn router() -> Router {
    let rpc_module = Arc::new(crate::rpc::YadTrackingRpcServer.module());
    Router::with_path("tracking")
        .hoop(affix::inject(rpc_module))
        .post(rpc_handler)
}

#[handler]
async fn rpc_handler(req: &mut Request, res: &mut Response, depot: &mut Depot) {
    let module = depot.obtain::<Arc<YadTrackingRpcModule>>();
    dbg!(&module);
    let payload = req
        .payload()
        .await
        .map(|msg| msg.as_slice())
        .and_then(|msg| from_utf8(msg).map_err(|err| ParseError::ParseFromStr));
    if let (Some(module), Ok(msg)) = (module, payload) {
        dbg!(&msg);
        dbg!(&module);
        let result = module.raw_json_request(msg).await;
        dbg!(&result);
        if let Ok((result, mut _stream)) = result {
            let result = result.result;
            res.set_status_code(StatusCode::OK);
            res.render(result);
            return;
        }
        return res.set_status_code(StatusCode::INTERNAL_SERVER_ERROR);
    }
    return res.set_status_code(StatusCode::INTERNAL_SERVER_ERROR);
}

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
