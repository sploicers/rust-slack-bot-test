use crate::util::SlackContext;
use async_trait::async_trait;
use slack_morphism::prelude::SlackMessageEvent;

#[async_trait]
pub trait MessageListener {
	fn applies_to_message(&self, message: &SlackMessageEvent) -> bool;
	async fn handle(&self, message: &SlackMessageEvent, ctx: &SlackContext<'_>);
}
