use super::robot::Robot;
use crate::util::{ApplicationConfig, Result};
use slack_morphism::prelude::*;
use std::sync::Arc;

pub async fn listen_for_slack_events(
	config: Arc<ApplicationConfig>,
	robot: Arc<Robot>,
) -> Result<()> {
	let client = Arc::new(SlackClient::new(SlackClientHyperConnector::new()));
	let callbacks = SlackSocketModeListenerCallbacks::new()
		.with_push_events(on_push_event)
		.with_command_events(on_command_event);

	let listener_environment = Arc::new(
		SlackClientEventsListenerEnvironment::new(client.clone())
			.with_user_state(robot.clone())
			.with_user_state(config.clone()),
	);

	let event_listener = SlackClientSocketModeListener::new(
		&SlackClientSocketModeConfig::new(),
		listener_environment,
		callbacks,
	);
	event_listener.listen_for(&config.slack_app_token).await?;
	event_listener.serve().await;
	Ok(())
}

async fn on_push_event(
	event: SlackPushEventCallback,
	client: Arc<SlackHyperClient>,
	state: SlackClientEventsUserState,
) -> Result<()> {
	let config = from_state::<ApplicationConfig>(&state).await;
	let robot = from_state::<Robot>(&state).await;
	let ctx = client.open_session(&config.slack_bot_token);

	match event.event {
		SlackEventCallbackBody::AppMention(mention) => {
			robot.handle_app_mention(&mention, &ctx, &config).await;
		}
		SlackEventCallbackBody::Message(message) => {
			robot.handle_message(&message, &ctx, &config).await;
		}
		_ => (),
	};
	Ok(())
}

async fn on_command_event(
	event: SlackCommandEvent,
	_client: Arc<SlackHyperClient>,
	_states: SlackClientEventsUserState,
) -> Result<SlackCommandEventResponse> {
	println!("{:#?}", event);
	Ok(SlackCommandEventResponse::new(
		SlackMessageContent::new().with_text("Working on it".into()),
	))
}

async fn from_state<T: Send + Sync + 'static>(state: &SlackClientEventsUserState) -> Arc<T> {
	state
		.read()
		.await
		.get_user_state::<Arc<T>>()
		.expect("Fatal - attempted to read unregistered type from user state.")
		.clone()
}
