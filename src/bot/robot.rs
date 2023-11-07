use super::MessageListener;
use crate::util::SlackContext;
use futures::future::join_all;
use slack_morphism::prelude::SlackMessageEvent;

pub struct Robot {
	listeners: Vec<Box<dyn MessageListener + Send + Sync + 'static>>,
}

impl Robot {
	pub fn new() -> Self {
		Self { listeners: vec![] }
	}

	pub fn with_listener(mut self, listener: impl MessageListener + Send + Sync + 'static) -> Self {
		self.listeners.push(Box::new(listener));
		self
	}

	pub async fn handle(&self, message: &SlackMessageEvent, ctx: &SlackContext<'_>) {
		join_all(self.listeners.iter().filter_map(|listener| {
			if listener.applies_to_message(message) {
				Some(listener.handle(message, ctx))
			} else {
				None
			}
		}))
		.await;
	}
}
