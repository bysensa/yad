use crate::app::YadAgentApp;
use flume::{bounded, Receiver, Sender};
use lazy_static::lazy_static;
use salvo::prelude::*;

lazy_static! {
    static ref SHUTDOWN_CHANNEL: (Sender<()>, Receiver<()>) = bounded(1);
}

pub struct YadAgentServer {}

impl YadAgentServer {
    pub fn new() -> Self {
        YadAgentApp::spawn(Self::init_server());
        YadAgentServer {}
    }

    async fn init_server() {
        let router = Router::new().get(hello_world);
        let server = Server::new(TcpListener::bind("127.0.0.1:7878")).serve_with_graceful_shutdown(
            router,
            async {
                SHUTDOWN_CHANNEL
                    .1
                    .recv_async()
                    .await
                    .expect("Shutdown signal received");
            },
        );
        server.await;
    }

    pub fn shutdown(&self) {
        let _ = async_std::task::block_on(SHUTDOWN_CHANNEL.0.send_async(()));
    }
}

#[handler]
async fn hello_world(res: &mut Response) {
    res.render("Hello world!");
}
