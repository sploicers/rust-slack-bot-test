use super::Listener;
use crate::util::{react_to_message, ApplicationConfig, SlackContext};
use async_trait::async_trait;
use rand::{rngs::ThreadRng, Rng};
use regex::Regex;
use slack_morphism::{prelude::SlackMessageEvent, SlackReactionName};
use std::sync::Arc;

pub struct NumberWanger {
	matcher: Regex,
}

impl NumberWanger {
	pub fn new() -> Self {
		Self {
			matcher: Regex::new(r"^\s*\d+\s*$").expect("Invalid regular expression."),
		}
	}
}

#[async_trait]
impl Listener<SlackMessageEvent> for NumberWanger {
	fn applies_to_event(&self, message: &SlackMessageEvent) -> bool {
		let is_numberwang_candidate = message
			.content
			.as_ref()
			.and_then(|content| content.text.as_ref())
			.map(|text| self.matcher.is_match(text))
			.unwrap_or(false);

		let is_numberwang = ThreadRng::default().gen_range(1..=10) <= 2;
		is_numberwang_candidate && is_numberwang
	}

	async fn handle(
		&self,
		message: &SlackMessageEvent,
		ctx: &SlackContext<'_>,
		_: &Arc<ApplicationConfig>,
	) {
		react_to_message(ctx, message, SlackReactionName::from("numberwang")).await;
	}
}
