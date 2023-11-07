use slack_morphism::{
	prelude::{
		SlackApiChatPostMessageRequest, SlackApiReactionsAddRequest,
		SlackClientHyperHttpsConnector, SlackMessageEvent,
	},
	SlackChannelId, SlackClientSession, SlackMessageContent, SlackReactionName,
};
pub type SlackContext<'a> = SlackClientSession<'a, SlackClientHyperHttpsConnector>;

pub async fn react_to_message(
	ctx: &SlackContext<'_>,
	message: &SlackMessageEvent,
	reaction: SlackReactionName,
) {
	let message_origin = message.origin.clone();
	let channel_id = message_origin.channel.unwrap();

	let _ = ctx
		.reactions_add(&SlackApiReactionsAddRequest::new(
			channel_id,
			reaction,
			message_origin.ts,
		))
		.await;
}

pub async fn post_in_channel(ctx: &SlackContext<'_>, channel_id: &SlackChannelId, text: &str) {
	let _ = ctx
		.chat_post_message(&SlackApiChatPostMessageRequest::new(
			channel_id.clone(),
			SlackMessageContent::new().with_text(text.to_string()),
		))
		.await;
}
