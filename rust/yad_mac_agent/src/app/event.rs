use crate::app::YadAgentApp;

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
        cacao::appkit::App::<YadAgentApp, Self>::dispatch_main(self)
    }
}
