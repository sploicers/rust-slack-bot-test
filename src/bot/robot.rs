use std::sync::Arc;

use super::{Listener, SlackEvent};
use crate::util::{ApplicationConfig, SlackContext};
use futures::future::join_all;
use slack_morphism::prelude::{SlackAppMentionEvent, SlackMessageEvent};

type RegisteredListeners<T> = Vec<Box<dyn Listener<T> + Send + Sync + 'static>>;

pub struct Robot {
	message_listeners: RegisteredListeners<SlackMessageEvent>,
	mention_listeners: RegisteredListeners<SlackAppMentionEvent>,
}

impl Robot {
	pub fn new() -> Self {
		Self {
			message_listeners: vec![],
			mention_listeners: vec![],
		}
	}

	pub fn with_message_listener(
		mut self,
		listener: impl Listener<SlackMessageEvent> + Send + Sync + 'static,
	) -> Self {
		self.message_listeners.push(Box::new(listener));
		self
	}

	pub fn with_app_mention_listener(
		mut self,
		listener: impl Listener<SlackAppMentionEvent> + Send + Sync + 'static,
	) -> Self {
		self.mention_listeners.push(Box::new(listener));
		self
	}

	pub async fn handle_message(
		&self,
		message: &SlackMessageEvent,
		ctx: &SlackContext<'_>,
		config: &Arc<ApplicationConfig>,
	) {
		self.fire_applicable_handlers(&self.message_listeners, message, ctx, config)
			.await
	}

	pub async fn handle_app_mention(
		&self,
		message: &SlackAppMentionEvent,
		ctx: &SlackContext<'_>,
		config: &Arc<ApplicationConfig>,
	) {
		self.fire_applicable_handlers(&self.mention_listeners, message, ctx, config)
			.await
	}

	async fn fire_applicable_handlers<T: SlackEvent>(
		&self,
		listeners: &RegisteredListeners<T>,
		event: &T,
		ctx: &SlackContext<'_>,
		config: &Arc<ApplicationConfig>,
	) {
		join_all(listeners.iter().filter_map(|listener| {
			if listener.applies_to_event(event) {
				Some(listener.handle(event, ctx, config))
			} else {
				None
			}
		}))
		.await;
	}
}
