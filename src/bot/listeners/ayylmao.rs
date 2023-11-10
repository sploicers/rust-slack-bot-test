use super::Listener;
use crate::util::{post_in_channel, ApplicationConfig, SlackContext};
use async_trait::async_trait;
use regex::Regex;
use slack_morphism::prelude::SlackMessageEvent;
use std::sync::Arc;

pub struct AyyLmao {
	matcher: Regex,
}

impl AyyLmao {
	pub fn new() -> Self {
		Self {
			matcher: Regex::new(r"\b(?i)AYY(Y*)\b").expect("Invalid regular expression."),
		}
	}
}

#[async_trait]
impl Listener<SlackMessageEvent> for AyyLmao {
	fn applies_to_event(&self, message: &SlackMessageEvent) -> bool {
		message
			.content
			.as_ref()
			.and_then(|content| content.text.as_ref())
			.map(|text| self.matcher.is_match(text))
			.unwrap_or(false)
	}

	async fn handle(
		&self,
		message: &SlackMessageEvent,
		ctx: &SlackContext<'_>,
		_: &Arc<ApplicationConfig>,
	) {
		let captures = message
			.content
			.as_ref()
			.and_then(|content| content.text.as_ref())
			.and_then(|text| self.matcher.captures(text));

		let channel_id = message.origin.channel.as_ref();

		if let (Some(captures), Some(channel_id)) = (captures, channel_id) {
			let y_count = captures[1].len();
			post_in_channel(ctx, channel_id, &format!("lma{}", "o".repeat(y_count + 1))).await
		}
	}
}
