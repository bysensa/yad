pub mod api;
mod app;

pub use app::AppState;
use entrait::Impl;

use api::AppRpcModule;

#[derive(Clone)]
pub struct App {
    state: Impl<app::AppState>,
}

impl App {
    pub fn new(state: AppState) -> Self {
        App { state: Impl::new(state) }
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_healthcheck() {
       
    }
}