pub mod activity;
mod types;

pub trait Entity {
    fn id(&self) -> Option<String>;

    fn collection(&self) -> String;
}
