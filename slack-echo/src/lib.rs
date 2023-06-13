use std::env;

use dotenv::dotenv;
use flowsnet_platform_sdk::logger;
use slack_flows::{listen_to_channel, send_message_to_channel, SlackMessage};

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    dotenv().ok();
    logger::init();

    let team = env::var("team").unwrap();
    let channel = env::var("channel").unwrap();

    log::info!("Running");
    listen_to_channel(&team, &channel, |msg| handler(msg, &team, &channel)).await;
    log::info!("Done");
}

async fn handler(msg: SlackMessage, team: &str, channel: &str) {
    log::info!("start callback");

    let text = msg.text;
    log::info!("received msg: {}", text);

    log::info!("sending msg: {}", text);
    send_message_to_channel(team, channel, text).await;
    log::info!("sended msg to {}/{}", team, channel);
}
