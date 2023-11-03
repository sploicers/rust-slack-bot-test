# Hubot RS

This is a work-in-progress Hubot replacement written in Rust. It makes use of the [slack-morphism](https://slack-rust.abdolence.dev/intro.html) crate.


## Building and running:
1. Follow the prerequisite steps [here](https://www.rust-lang.org/tools/install).
2. Obtain a .env file with values for the Slack API token, etc., and place it in the root directory.
3. `cargo build`
4. `cargo run`

Alternatively, you can use Docker (this will take a while the first time):
1. `docker build . -t hubot-rs`
2. `docker run --rm hubot-rs`