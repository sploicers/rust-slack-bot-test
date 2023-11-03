use crate::util::Result;
use async_trait::async_trait;
use slack_morphism::{
	prelude::{SlackClientHyperHttpsConnector, SlackMessageEvent},
	SlackClientSession,
};
pub type SlackContext<'a> = SlackClientSession<'a, SlackClientHyperHttpsConnector>;

#[async_trait]
pub trait BotCommand {
	async fn execute(&self, ctx: &SlackContext) -> Result<()>;
}

pub trait FromSlackMessage<T> {
	fn from_message(message: SlackMessageEvent) -> Option<T>
	where
		T: BotCommand;
}
