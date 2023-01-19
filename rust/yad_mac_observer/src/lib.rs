mod common;

pub use common::{ApplicationName, BundleId, DateTime, MacEventObserver, MacObservedEvent, Pid};

/*
Usage example

fn main() {

    unsafe { NSAutoreleasePool::new(nil);
        let app = NSApp();

        let mut watcher = ApplicationActivationWatcher::new();
        watcher.start();
        app.run();
    }
}

 */
