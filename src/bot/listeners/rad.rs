use super::MessageListener;
use crate::bot::context::SlackContext;
use async_trait::async_trait;
use regex::Regex;
use slack_morphism::{
	prelude::{SlackApiReactionsAddRequest, SlackMessageEvent},
	SlackReactionName,
};

pub struct RadListener {
	matcher: Regex,
}

impl RadListener {
	pub fn new() -> Self {
		Self {
			matcher: Regex::new(r"\b(rad)\b").expect("Invalid regular expression."),
		}
	}
}

#[async_trait]
impl MessageListener for RadListener {
	fn applies_to_message(&self, message: &SlackMessageEvent) -> bool {
		message
			.content
			.as_ref()
			.and_then(|content| content.text.as_ref())
			.map(|text| self.matcher.is_match(text))
			.unwrap_or(false)
	}

	async fn handle(&self, message: &SlackMessageEvent, ctx: &SlackContext<'_>) {
		let message_origin = message.origin.clone();
		let channel_id = message_origin.channel.unwrap();

		let _ = ctx
			.reactions_add(&SlackApiReactionsAddRequest::new(
				channel_id,
				SlackReactionName::from("call_me_hand"),
				message_origin.ts,
			))
			.await;
	}
}
