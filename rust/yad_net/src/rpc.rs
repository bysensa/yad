use jsonrpsee::RpcModule;
use salvo::http::ParseError;
use salvo::prelude::*;
use std::str::from_utf8;
use std::sync::Arc;

pub trait RpcReply {
    type Output: Send + Sync + 'static;
}

pub async fn handler<T>(module: Option<&Arc<RpcModule<T>>>, req: &mut Request, res: &mut Response) {
    let payload = req
        .payload()
        .await
        .map(|msg| msg.as_slice())
        .and_then(|msg| from_utf8(msg).map_err(|_| ParseError::ParseFromStr));
    if let (Some(module), Ok(msg)) = (module, payload) {
        if let Ok((result, mut _stream)) = module.raw_json_request(msg).await {
            let result = result.result;
            res.set_status_code(StatusCode::OK);
            res.render(result);
            return;
        }
        return res.set_status_code(StatusCode::INTERNAL_SERVER_ERROR);
    }
    return res.set_status_code(StatusCode::INTERNAL_SERVER_ERROR);
}
