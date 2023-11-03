use crate::commands::botcommand::{BotCommand, SlackContext};
use crate::commands::FromSlackMessage;
use crate::util::Result;
use async_trait::async_trait;
use rand::seq::SliceRandom;
use slack_morphism::prelude::{SlackApiChatPostMessageRequest, SlackMessageEvent};
use slack_morphism::{SlackChannelId, SlackMessageContent};

const ALOT_OF_URLS: &'static [&'static str] = &[
	"https://3.bp.blogspot.com/_D_Z-D2tzi14/S8TffVGLElI/AAAAAAAACxA/trH1ch0Y3tI/s320/ALOT6.png",
	"http://1.bp.blogspot.com/_D_Z-D2tzi14/S8TflwXvTgI/AAAAAAAACxI/qgd1wYcTWV8/s320/ALOT12.png",
	"https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcS2mynEom5JcAZCTGQPmDCfL7rFqDCDn9Dkq03ePZGl14w9bpjCJxUWL09ZqEeV2eRJJsA&usqp=CAU",
	"https://i.kym-cdn.com/photos/images/original/000/177/517/ALOT15.png"
];

pub struct AlotCommand {
	pub channel_id: SlackChannelId,
	pub img_url: String,
}

impl FromSlackMessage<AlotCommand> for AlotCommand {
	fn from_message(message: SlackMessageEvent) -> Option<Self> {
		let matches = message
			.content
			.and_then(|content| content.text)
			.map(|text| text.contains("alot"))
			.unwrap_or_default();

		if matches {
			let channel_id = message.origin.channel.unwrap();
			let img_url = String::from(*ALOT_OF_URLS.choose(&mut rand::thread_rng()).unwrap());
			Some(AlotCommand {
				channel_id,
				img_url,
			})
		} else {
			None
		}
	}
}

#[async_trait]
impl BotCommand for AlotCommand {
	async fn execute(&self, ctx: &SlackContext) -> Result<()> {
		let result = ctx
			.chat_post_message(&SlackApiChatPostMessageRequest::new(
				self.channel_id.clone(),
				SlackMessageContent::new().with_text(self.img_url.clone()),
			))
			.await;

		match result {
			Err(e) => Ok(()),
			_ => Ok(()),
		}
	}
}
