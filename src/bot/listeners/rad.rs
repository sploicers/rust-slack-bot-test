use std::sync::Arc;

use super::Listener;
use crate::util::{react_to_message, ApplicationConfig, SlackContext};
use async_trait::async_trait;
use regex::Regex;
use slack_morphism::{prelude::SlackMessageEvent, SlackReactionName};

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
impl Listener<SlackMessageEvent> for RadListener {
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
		react_to_message(ctx, message, SlackReactionName::from("call_me_hand")).await;
	}
}
