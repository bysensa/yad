use std::thread;
use yad_mac_observer::MacEventObserver;

#[derive(Default)]
struct ExampleApp {}

impl cacao::appkit::AppDelegate for ExampleApp {}

fn main() {
    let app = cacao::appkit::App::new("dev.bysensa.yad", ExampleApp::default());
    let event_receiver = MacEventObserver::receiver();
    let mut observer = MacEventObserver::new();
    observer.start();
    let _handle = thread::spawn(move || {
        while let Ok(event) = event_receiver.recv() {
            dbg!(event);
        }
    });
    app.run();
}
