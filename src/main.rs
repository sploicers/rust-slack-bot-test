use bot::listen_for_slack_events;
use std::sync::Arc;
use util::{ApplicationConfig, Result};
mod bot;
mod commands;
mod util;

#[tokio::main]
async fn main() -> Result<()> {
	let config = Arc::new(ApplicationConfig::new()?);
	listen_for_slack_events(config).await?;
	Ok(())
}
