use crate::{
	commands::{AlotCommand, BotCommand, FromSlackMessage},
	util::{ApplicationConfig, Result},
};
use slack_morphism::prelude::*;
use std::sync::Arc;

type SlackSession<'a> = SlackClientSession<'a, SlackClientHyperHttpsConnector>;

pub async fn listen_for_slack_events(config: Arc<ApplicationConfig>) -> Result<()> {
	let client = Arc::new(SlackClient::new(SlackClientHyperConnector::new()));
	let callbacks = SlackSocketModeListenerCallbacks::new()
		.with_command_events(on_command_event)
		.with_interaction_events(on_interaction_event)
		.with_push_events(on_push_event);

	let listener_environment = Arc::new(
		SlackClientEventsListenerEnvironment::new(client.clone()).with_user_state(config.clone()),
	);
	let event_listener = SlackClientSocketModeListener::new(
		&SlackClientSocketModeConfig::new(),
		listener_environment,
		callbacks,
	);
	event_listener.listen_for(&config.slack_api_token).await?;
	event_listener.serve().await;
	Ok(())
}

async fn on_interaction_event(
	event: SlackInteractionEvent,
	_client: Arc<SlackHyperClient>,
	_states: SlackClientEventsUserState,
) -> Result<()> {
	println!("{:#?}", event);
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

async fn on_push_event(
	event: SlackPushEventCallback,
	client: Arc<SlackHyperClient>,
	state: SlackClientEventsUserState,
) -> Result<()> {
	let config = state
		.read()
		.await
		.get_user_state::<Arc<ApplicationConfig>>()
		.expect("Fatal - app configuration not registered in user state.")
		.clone();

	match event.event {
		SlackEventCallbackBody::Message(message_event) => {
			let session: SlackSession = client.open_session(&config.slack_bot_token);

			if let Some(command) = AlotCommand::from_message(message_event) {
				command.execute(&session).await?
			}
			Ok(())
		}
		SlackEventCallbackBody::AppMention(mention_event) => {
			println!("{:#?}", mention_event);
			Ok(())
		}
		_ => Ok(()),
	}
}
