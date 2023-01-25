mod event;
mod observer;
mod server;

use crate::app::event::MacObservedEvent;
use crate::app::server::YadAgentServer;
use async_std::task::JoinHandle;
use cacao::appkit::*;
use cacao::notification_center::Dispatcher;
use observer::MacEventObserver;
use std::cell::RefCell;
use std::future::Future;

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
        self.server.shutdown();
    }
}

impl Dispatcher for YadAgentApp {
    type Message = MacObservedEvent;

    fn on_ui_message(&self, message: Self::Message) {
        dbg!(&message);
    }
}
