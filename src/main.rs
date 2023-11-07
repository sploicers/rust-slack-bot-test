use bot::{listen_for_slack_events, AlotListener, NumberWanger, RadListener, Robot};
use std::sync::Arc;
use util::{ApplicationConfig, Result};
mod bot;
mod util;

#[tokio::main]
async fn main() -> Result<()> {
	let config = ApplicationConfig::new()?;
	let robot = Robot::new()
		.with_listener(AlotListener::new())
		.with_listener(RadListener::new())
		.with_listener(NumberWanger::new());

	listen_for_slack_events(Arc::new(config), Arc::new(robot)).await?;
	Ok(())
}
