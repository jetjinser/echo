use std::env;

use slack_flows::{listen_to_channel, send_message_to_channel, SlackMessage};

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    let team = env::var("team").unwrap();
    let channel = env::var("channel").unwrap();

    listen_to_channel(&team, &channel, |msg| handler(msg, &team, &channel)).await;
}

async fn handler(msg: SlackMessage, team: &str, channel: &str) {
    let text = msg.text;
    send_message_to_channel(team, channel, text).await;
}
