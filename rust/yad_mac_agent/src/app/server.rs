use crate::app::YadAgentApp;
// use crate::RUNTIME;
use async_std::task::JoinHandle;
use flume::{bounded, Receiver, Sender};
use lazy_static::lazy_static;
use salvo::prelude::*;

lazy_static! {
    static ref SHUTDOWN_CHANNEL: (Sender<()>, Receiver<()>) = bounded(1);
}

pub struct YadAgentServer {
    handle: JoinHandle<()>,
}

impl YadAgentServer {
    pub fn new() -> Self {
        let handle = YadAgentApp::spawn(Self::init_server());
        YadAgentServer { handle }
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

    pub fn shutdown() {
        SHUTDOWN_CHANNEL.0.send_async(());
    }
}

#[handler]
async fn hello_world(res: &mut Response) {
    res.render("Hello world!");
}
