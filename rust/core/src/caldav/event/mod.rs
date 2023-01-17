use messages::prelude::*;
use messages::address::Address;

#[cfg(test)]
mod tests {
	use messages::prelude::*;
	use crate::event::{Ev, EvApi};
	
	#[tokio::test]
	async fn actor_spawn_test() {
		let addr = Ev{}.spawn();
		addr.echo().await;
	}
}

struct CreateEvent {}

impl CreateEvent {

}


struct Ev {}


impl Actor for Ev {}

#[async_trait]
pub trait EvApi {
	async fn echo(&self) {
		println!("hello");
	}
}

impl EvApi for Address<Ev> {

}