use super::Listener;
use crate::util::{post_in_channel, ApplicationConfig, SlackContext};
use async_trait::async_trait;
use regex::Regex;
use slack_morphism::prelude::SlackMessageEvent;
use std::sync::Arc;

pub struct PlusPlusListener {
	matcher: Regex,
}

impl PlusPlusListener {
	pub fn new() -> Self {
		let nominee = r"^(?<nominee>[\S']+)?(?:[\W\s]*)?";
		let increment_decrement = r"(?<operation>\+\+|--)";
		let reason_prefix = "(?:for|because|cause|cuz|coz)";
		let reason = format!(r"{}\s+(?<reason>.+)?$", reason_prefix);
		let full_regex = format!(r"{}{}{}", nominee, increment_decrement, reason);
		let matcher = Regex::new(&full_regex).expect("Invalid regular expression.");
		Self { matcher }
	}
}

#[async_trait]
impl Listener<SlackMessageEvent> for PlusPlusListener {
	fn applies_to_event(&self, mention: &SlackMessageEvent) -> bool {
		let applies = mention
			.content
			.as_ref()
			.and_then(|content| content.text.as_ref())
			.map(|text| self.matcher.is_match(text))
			.unwrap_or(false);
		println!("{}", applies);
		applies
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
			.map(|text| self.matcher.captures(text))
			.flatten()
			.expect("We already know that the message matches if we get to this point.");

		let nominee = &captures["nominee"];
		let action = &captures["action"];
		let reason_prefix = &captures["reason_prefix"];
		let reason = &captures["reason"];

		let response = match action {
			"++" => format!("{} gets a point {} {}", nominee, reason_prefix, reason),
			"--" => format!("{} loses a point {} {}", nominee, reason_prefix, reason),
			_ => panic!(),
		};

		println!("{}", response);
		let channel_id = message.origin.channel.as_ref().unwrap();
		post_in_channel(ctx, channel_id, &response).await;
	}
}
