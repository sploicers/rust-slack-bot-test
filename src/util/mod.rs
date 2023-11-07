mod config;
mod result;
mod slack_helpers;
pub use config::ApplicationConfig;
pub use result::{Error, Result};
pub use slack_helpers::{post_in_channel, react_to_message, SlackContext};
