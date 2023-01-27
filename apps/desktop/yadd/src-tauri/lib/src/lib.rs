use messages::{prelude::{Actor, Notifiable, Context}, registry::{Service, Registry}};

mod datastore;
mod platform;

pub use platform::mac::MacEventObserver;


pub struct AppCore {}

impl Default for AppCore {
    fn default() -> Self {
        Self {  }
    }
}

impl Actor for AppCore {}

impl Service for AppCore {
    const NAME: &'static str = "APP";
}

#[async_trait::async_trait]
impl Notifiable<MacObservedEvent> for AppCore {
    async fn notify(&mut self, input: MacObservedEvent, context: &Context<Self>) {
        
    }
}





pub type DateTime = chrono::DateTime<chrono::Utc>;
pub type ApplicationName = String;
pub type BundleId = String;
pub type Pid = i64;

#[derive(Debug)]
pub enum MacObservedEvent {
    ApplicationActivated(Pid, BundleId, ApplicationName, DateTime),
    ScreenLocked,
    ScreenUnlocked,
}

impl MacObservedEvent {
    pub fn dispatch(self) {
        async_std::task::block_on(async {
            let mut addr = Registry::service::<AppCore>().await;
            addr.notify(self).await;
        });
    }
}
