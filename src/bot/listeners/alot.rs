use std::sync::Arc;

use super::Listener;
use crate::util::{post_in_channel, ApplicationConfig, SlackContext};
use async_trait::async_trait;
use rand::seq::SliceRandom;
use regex::Regex;
use slack_morphism::prelude::SlackMessageEvent;

const ALOT_OF_URLS: &[&str] = &[
	"https://3.bp.blogspot.com/_D_Z-D2tzi14/S8TffVGLElI/AAAAAAAACxA/trH1ch0Y3tI/s320/ALOT6.png",
	"http://1.bp.blogspot.com/_D_Z-D2tzi14/S8TflwXvTgI/AAAAAAAACxI/qgd1wYcTWV8/s320/ALOT12.png",
	"https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcS2mynEom5JcAZCTGQPmDCfL7rFqDCDn9Dkq03ePZGl14w9bpjCJxUWL09ZqEeV2eRJJsA&usqp=CAU",
	"https://i.kym-cdn.com/photos/images/original/000/177/517/ALOT15.png"
];

pub struct AlotListener {
	matcher: Regex,
}

impl AlotListener {
	pub fn new() -> Self {
		Self {
			matcher: Regex::new(r"\b(alot)\b").expect("Invalid regular expression."),
		}
	}
}

#[async_trait]
impl Listener<SlackMessageEvent> for AlotListener {
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
		let channel_id = message.origin.channel.as_ref().unwrap();
		let img_url = String::from(
			*ALOT_OF_URLS
				.choose(&mut rand::thread_rng())
				.expect("Failed to select image URL from list."),
		);
		post_in_channel(ctx, channel_id, &img_url).await;
	}
}
