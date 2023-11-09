use crate::util::{ApplicationConfig, Result};
use slack_morphism::SlackUserId;
use std::sync::Arc;
extern crate redis;

pub struct Brain {
	config: Arc<ApplicationConfig>,
	memory: Arc<redis::Client>,
}

pub struct PointsAwardedEvent {
	recipient: SlackUserId,
	reason: String,
}

pub enum MemorableEvent {
	PointsAwarded(PointsAwardedEvent),
}

impl Brain {
	pub fn new(config: Arc<ApplicationConfig>) -> Result<Self> {
		let memory = Arc::new(redis::Client::open(config.redis_conn_str.clone())?);
		Ok(Self { config, memory })
	}

	pub fn memorize(&self, thing: MemorableEvent) {
		match thing {
			MemorableEvent::PointsAwarded(data) => {
				self.store_points(data);
			}
			_ => unimplemented!(),
		}
	}

	pub fn retrieve(&self) -> MemorableEvent {
		unimplemented!()
	}

	fn store_points(&self, data: PointsAwardedEvent) -> Result<()> {
		unimplemented!();
		let conn = self.memory.get_connection()?;
	}
}
