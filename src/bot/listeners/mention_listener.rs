use crate::util::SlackContext;
use async_trait::async_trait;
use slack_morphism::prelude::SlackAppMentionEvent;

#[async_trait]
pub trait MentionListener {
	fn applies_to_message(&self, mention: &SlackAppMentionEvent) -> bool;
	async fn handle(&self, message: &SlackAppMentionEvent, ctx: &SlackContext<'_>);
}
