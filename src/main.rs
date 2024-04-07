pub mod db;
pub mod event;
pub mod framework;
pub mod grubber;
pub mod threader;
pub mod ui;

use poise::serenity_prelude as serenity;
use std::env;

use dotenv::dotenv;

use crate::db::race::race;

use self::event::event_handler;

pub struct Data {
    pub mongo: db::MongoDB,
}
pub(crate) type Error = Box<dyn std::error::Error + Send + Sync>;
pub(crate) type Context<'a> = poise::Context<'a, Data, Error>;
pub(crate) type ApplicationContext<'a> = poise::ApplicationContext<'a, Data, Error>;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();
    let token = env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN in the environment");
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::<Data, Error>::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                framework::register_commands_buttons::register(),
                grubber::channel_subsequent::move_channel_subsequent(),
            ],
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .setup(|_ctx, _ready, _framework| {
            Box::pin(async move {
                let mongo = db::MongoDB::connect().await?;
                Ok(Data { mongo })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}
