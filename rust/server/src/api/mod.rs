
use salvo::{Router, affix};

use crate::app::App;


mod rpc;


pub fn router(app: App) -> Router {
    Router::with_path("api").hoop(affix::inject(app)).push(rpc::router())
}