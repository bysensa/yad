use entrait::Impl;

use salvo::prelude::*;
use server::{db::Database, app::{App, AppState}};


#[tokio::main(flavor = "current_thread")]
async fn main() {
    let db = Database::new("memory", None, None).await.unwrap();
    let state = AppState::new(db);
    let app = App::new(state);
    let app = Impl::new(app);
    let router = server::api::router(app);
    Server::new(TcpListener::bind("127.0.0.1:7878")).serve(router).await;
}
