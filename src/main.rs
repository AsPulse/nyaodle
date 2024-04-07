pub mod db;
pub mod framework;
pub mod grubber;
pub mod ui;

use log::info;
use poise::serenity_prelude as serenity;
use std::env;

use dotenv::dotenv;

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

async fn event_handler(
    _ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {
    if let serenity::FullEvent::Ready { data_about_bot, .. } = event {
        info!("Logged in as {}", data_about_bot.user.name);
    }
    Ok(())
}
