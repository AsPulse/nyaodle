use log::info;
use poise::serenity_prelude::{self as serenity, CreateWebhook, MessageFlags};
use poise::serenity_prelude::{ChannelId, ExecuteWebhook};

use crate::threader::{MessageBulk, ThreaderState};
use crate::Error;

use super::Threader;

pub struct AnotherChannelThreader<'a> {
    pub ctx: &'a serenity::Context,
    pub channel_id: ChannelId,
}

impl Threader for AnotherChannelThreader<'_> {
    async fn thread(
        &self,
        id: &str,
        tx: tokio::sync::mpsc::Sender<super::ThreaderMessage>,
        mut rx: tokio::sync::mpsc::Receiver<super::MessageBulk>,
    ) -> Result<(), Error> {
        info!("New another_channel threader started with id={}", id);

        let Some(channel) = self.ctx.http.get_channel(self.channel_id).await?.guild() else {
            panic!("Specified channel is not a guild channel. (maybe DM channel)");
        };
        let webhook = channel
            .create_webhook(
                self.ctx,
                CreateWebhook::new(format!("nyaodle-threader-{}", id)),
            )
            .await?;
        let mut state = ThreaderState {
            num_threaded_messages: 0,
            is_completed: false,
        };
        let ctx = self.ctx.clone();
        tokio::spawn(async move {
            loop {
                let Some(MessageBulk::Continue(message)) = rx.recv().await else {
                    state.is_completed = true;
                    tx.send(super::ThreaderMessage::StateUpdate(state.clone()))
                        .await
                        .unwrap();
                    break;
                };

                let execute = ExecuteWebhook::new()
                    .username(
                        &message
                            .author_nick(&ctx)
                            .await
                            .unwrap_or(message.author.name.clone()),
                    )
                    .avatar_url(message.author.avatar_url().unwrap_or_default())
                    .content(message.content)
                    .flags(MessageFlags::SUPPRESS_NOTIFICATIONS);

                webhook.execute(&ctx, false, execute).await.unwrap();
            }
        });
        Ok(())
    }
}
