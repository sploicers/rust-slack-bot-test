use slack_morphism::{prelude::SlackClientHyperHttpsConnector, SlackClientSession};
pub type SlackContext<'a> = SlackClientSession<'a, SlackClientHyperHttpsConnector>;
