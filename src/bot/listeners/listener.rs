use crate::util::{ApplicationConfig, SlackContext};
use async_trait::async_trait;
use slack_morphism::prelude::{SlackAppMentionEvent, SlackMessageEvent};
use std::sync::Arc;

pub trait SlackEvent {}
impl SlackEvent for SlackMessageEvent {}
impl SlackEvent for SlackAppMentionEvent {}

#[async_trait]
pub trait Listener<T: SlackEvent> {
	fn applies_to_event(&self, message: &T) -> bool;

	async fn handle(&self, message: &T, ctx: &SlackContext<'_>, config: &Arc<ApplicationConfig>);
}
