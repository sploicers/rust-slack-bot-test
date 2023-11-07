use crate::util::{post_in_channel, SlackContext};

use super::MentionListener;
use async_trait::async_trait;
use rand::Rng;
use regex::Regex;
use slack_morphism::{
	prelude::{SlackApiConversationsMembersRequest, SlackAppMentionEvent},
	SlackChannelId, SlackUserId,
};

pub struct WhoListener {
	matcher: Regex,
}

impl WhoListener {
	pub fn new() -> Self {
		Self {
			matcher: Regex::new(r"who should (?<thething>.+)\?$")
				.expect("Invalid regular expression."),
		}
	}
}

#[async_trait]
impl MentionListener for WhoListener {
	fn applies_to_message(&self, mention: &SlackAppMentionEvent) -> bool {
		mention
			.content
			.text
			.as_ref()
			.map(|text| self.matcher.is_match(text))
			.unwrap_or(false)
	}

	async fn handle(&self, message: &SlackAppMentionEvent, ctx: &SlackContext<'_>) {
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
		let chosen_user = random_user_from_channel(ctx, channel_id.clone()).await;
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

async fn random_user_from_channel(
	ctx: &SlackContext<'_>,
	channel_id: SlackChannelId,
) -> SlackUserId {
	let mut users = list_users_in_channel(ctx, channel_id).await;
	let mut rng = rand::thread_rng();
	users.swap_remove(rng.gen_range(0..users.len()))
}
