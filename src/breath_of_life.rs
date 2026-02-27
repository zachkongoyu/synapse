use crate::agent::Agent;

pub trait Life {
	async fn perceive();

	async fn think();

	async fn act();

	async fn live() {
		loop {
			Self::perceive().await;
			Self::think().await;
			Self::act().await;
		}
	}
}

impl Life for Agent {
	async fn perceive() {
		todo!()
	}

	async fn think() {
		todo!()
	}

	async fn act() {
		todo!()
	}
}

pub fn assert_life_impl<T: Life>() {}

