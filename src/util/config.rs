use super::Result;
use dotenv::dotenv;
use slack_morphism::{SlackApiToken, SlackApiTokenValue, SlackUserId};
extern crate dotenv;

pub struct ApplicationConfig {
	pub slack_app_id: String,
	pub slack_client_id: String,
	pub slack_client_secret: String,
	pub slack_signing_secret: String,
	pub slack_app_token: SlackApiToken,
	pub slack_bot_token: SlackApiToken,
	pub slack_bot_user_id: SlackUserId,
	pub redis_conn_str: String,
}

impl ApplicationConfig {
	pub fn new() -> Result<Self> {
		dotenv()?;
		Ok(Self {
			slack_app_id: envar_or_panic("SLACK_APPLICATION_ID"),
			slack_client_id: envar_or_panic("SLACK_CLIENT_ID"),
			slack_client_secret: envar_or_panic("SLACK_CLIENT_SECRET"),
			slack_signing_secret: envar_or_panic("SLACK_SIGNING_SECRET"),
			slack_app_token: slack_token_from_env("SLACK_APP_LEVEL_TOKEN"),
			slack_bot_token: slack_token_from_env("SLACK_BOT_USER_OAUTH_TOKEN"),
			slack_bot_user_id: SlackUserId(envar_or_panic("SLACK_BOT_USER_ID")),
			redis_conn_str: build_redis_conn_str(),
		})
	}
}

fn envar_or_panic(key: &str) -> String {
	std::env::var(key).expect(&format!(
		"'{}' environment variable is required, but wasn't set.",
		key
	))
}

fn slack_token_from_env(key: &str) -> SlackApiToken {
	SlackApiToken::new(SlackApiTokenValue(envar_or_panic(key)))
}

fn build_redis_conn_str() -> String {
	format!(
		"redis://{}@{}:{}/{}",
		envar_or_panic("REDIS_PASSWORD"),
		envar_or_panic("REDIS_HOSTNAME"),
		envar_or_panic("REDIS_PORT"),
		envar_or_panic("REDIS_DATABASE")
	)
}
