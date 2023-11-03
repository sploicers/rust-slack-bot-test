use super::Result;
use dotenv::dotenv;
use slack_morphism::{SlackApiToken, SlackApiTokenValue};
extern crate dotenv;

pub struct ApplicationConfig {
	pub slack_app_id: String,
	pub slack_client_id: String,
	pub slack_client_secret: String,
	pub slack_api_token: SlackApiToken,
	pub slack_bot_token: SlackApiToken,
}

impl ApplicationConfig {
	pub fn new() -> Result<Self> {
		dotenv()?;
		Ok(Self {
			slack_app_id: envar_or_panic("SLACK_APPLICATION_ID"),
			slack_client_id: envar_or_panic("SLACK_CLIENT_ID"),
			slack_client_secret: envar_or_panic("SLACK_CLIENT_SECRET"),
			slack_api_token: SlackApiToken::new(SlackApiTokenValue(envar_or_panic(
				"SLACK_APPLICATION_TOKEN",
			))),
			slack_bot_token: SlackApiToken::new(SlackApiTokenValue(envar_or_panic(
				"SLACK_BOT_TOKEN",
			))),
		})
	}
}

fn envar_or_panic(key: &str) -> String {
	std::env::var(key).expect(&format!(
		"'{}' environment variable is required, but wasn't set.",
		key
	))
}
