mod server;

use crate::app::server::YadAgentServer;
use async_std::task::JoinHandle;
use cacao::appkit::*;
use lazy_static::lazy_static;
use std::cell::RefCell;
use std::future::Future;
use yad_mac_observer::MacEventObserver;

pub struct YadAgentApp {
    observer: RefCell<MacEventObserver>,
    server: YadAgentServer,
}

impl YadAgentApp {
    pub fn new() -> Self {
        YadAgentApp {
            observer: RefCell::new(MacEventObserver::new()),
            server: YadAgentServer::new(),
        }
    }

    pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        async_std::task::spawn(future)
    }
}

impl AppDelegate for YadAgentApp {
    fn did_finish_launching(&self) {
        self.observer.borrow_mut().start();
    }

    fn will_terminate(&self) {
        self.observer.borrow_mut().stop();
    }
}
