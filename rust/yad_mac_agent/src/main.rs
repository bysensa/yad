mod app;

// use crate::app::RUNTIME;
use cacao::appkit::*;
use salvo::prelude::*;
use std::thread;
use yad_mac_observer::MacEventObserver;

fn main() {
    let mut app = App::new("dev.bysensa.yad.agent", app::YadAgentApp::new());
    thread::spawn(|| {
        let receiver = MacEventObserver::receiver();
        while let Ok(event) = receiver.recv() {
            dbg!(event);
        }
    });
    app.run();
}
