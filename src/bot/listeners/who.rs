use super::Listener;
use crate::util::{post_in_channel, ApplicationConfig, SlackContext};
use async_trait::async_trait;
use rand::{rngs::ThreadRng, Rng};
use regex::Regex;
use slack_morphism::{
	prelude::{SlackApiConversationsMembersRequest, SlackAppMentionEvent},
	SlackChannelId, SlackUserId,
};
use std::sync::Arc;

pub struct WhoListener {
	matcher: Regex,
}

impl WhoListener {
	pub fn new() -> Self {
		Self {
			matcher: Regex::new(r"[Ww]ho should (?<thething>.+)\?$")
				.expect("Invalid regular expression."),
		}
	}
}

#[async_trait]
impl Listener<SlackAppMentionEvent> for WhoListener {
	fn applies_to_event(&self, mention: &SlackAppMentionEvent) -> bool {
		mention
			.content
			.text
			.as_ref()
			.map(|text| self.matcher.is_match(text))
			.unwrap_or(false)
	}

	async fn handle(
		&self,
		message: &SlackAppMentionEvent,
		ctx: &SlackContext<'_>,
		config: &Arc<ApplicationConfig>,
	) {
		let msg_text = message
			.content
			.text
			.as_ref()
			.expect("We've already checked that the message has content.");

		let captures = self
			.matcher
			.captures(&msg_text)
			.expect("We've already checked if the message matches.");

		let channel_id = message.channel.clone();
		let thing_to_do = &captures["thething"];

		let mut users_in_channel: Vec<SlackUserId> = list_users_in_channel(ctx, channel_id.clone())
			.await
			.into_iter()
			.filter(|user_id| user_id != &config.slack_bot_user_id)
			.collect();

		let chosen_user =
			users_in_channel.swap_remove(ThreadRng::default().gen_range(0..users_in_channel.len()));

		let response = format!("<@{}> should {}.", chosen_user, thing_to_do);
		post_in_channel(ctx, &channel_id, &response).await;
	}
}

async fn list_users_in_channel(
	ctx: &SlackContext<'_>,
	channel_id: SlackChannelId,
) -> Vec<SlackUserId> {
	ctx.conversations_members(&SlackApiConversationsMembersRequest::new().channel(channel_id))
		.await
		.map(|res| res.members)
		.unwrap_or_default()
}
