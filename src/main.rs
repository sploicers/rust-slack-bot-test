use bot::{
	listen_for_slack_events, AlotListener, AyyLmao, NumberWanger, RadListener, Robot, WhoListener,
};
use std::sync::Arc;
use util::{ApplicationConfig, Result};
mod bot;
mod util;

#[tokio::main]
async fn main() -> Result<()> {
	let config = Arc::new(ApplicationConfig::new()?);
	let robot = Robot::new()
		.with_message_listener(AlotListener::new())
		.with_message_listener(RadListener::new())
		.with_message_listener(NumberWanger::new())
		.with_message_listener(AyyLmao::new())
		.with_app_mention_listener(WhoListener::new());

	listen_for_slack_events(config, Arc::new(robot)).await?;
	Ok(())
}
