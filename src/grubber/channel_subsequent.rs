//! あるチャンネルに投稿された、このメッセージ以降のメッセージを対象にする

use crate::db::race::race_interaction;
use crate::grubber::GrubberMessage;
use crate::grubber::GrubberState;
use crate::grubber::NyaodleRequest;
use crate::threader::MessageBulk;
use crate::ui::configure_threader::create::configure_threader;
use log::info;
use log::warn;
use poise::command;
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::ChannelId;
use poise::serenity_prelude::GetMessages;
use poise::serenity_prelude::GuildChannel;
use poise::serenity_prelude::Message;
use poise::serenity_prelude::MessageId;
use serde::Deserialize;
use serde::Serialize;

use crate::ApplicationContext;
use crate::Error;

use super::Grubber;

#[command(context_menu_command = "これ以降のメッセージを移動")]
pub async fn move_channel_subsequent(
    ctx: ApplicationContext<'_>,
    message: serenity::Message,
) -> Result<(), Error> {
    if race_interaction(&ctx).await? {
        return Ok(());
    }
    let request = NyaodleRequest::ChannelSubsequent(ChannelSubsequent {
        message_id: message.id,
        channel_id: message.channel_id,
    });
    configure_threader(ctx, request).await?;
    Ok(())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelSubsequent {
    message_id: MessageId,
    channel_id: ChannelId,
}

pub struct ChannelSubsequentGrubber<'a> {
    pub ctx: &'a serenity::Context,
    pub property: ChannelSubsequent,
}

impl Grubber for ChannelSubsequentGrubber<'_> {
    async fn grub(
        &self,
        id: &str,
        tx: tokio::sync::mpsc::Sender<super::GrubberMessage>,
        message_tx: tokio::sync::mpsc::Sender<MessageBulk>,
    ) -> Result<(), Error> {
        let ctx = self.ctx.clone();
        info!("New channel_subsequent grubber started with id={}", id);
        let Some(channel) = ctx
            .http
            .get_channel(self.property.channel_id)
            .await?
            .guild()
        else {
            panic!("Specified channel is not a guild channel. (maybe DM channel)");
        };
        let mut state = GrubberState {
            num_total_messages: 1,
            num_grubbed_messages: 1,
            is_completed: false,
        };
        info!("Channel_subsequent grubber loading message_start id={}", id);
        let message_start = ctx
            .http
            .get_message(self.property.channel_id, self.property.message_id)
            .await?;
        let mut message_pointer = message_start.clone();
        info!(
            "Channel_subsequent grubber loading message_end id={}, message_id={:?}",
            id, channel.last_message_id
        );
        let message_end = retrieve_final_message(&ctx, &channel).await?;

        message_tx
            .send(MessageBulk::Continue(message_pointer.clone()))
            .await
            .unwrap();
        let id = id.to_string();
        tokio::spawn(async move {
            let ctx = &ctx;
            loop {
                if let (Some(pointer_position), Some(end_position)) =
                    (message_pointer.position, message_end.position)
                {
                    let approximate_total =
                        end_position - pointer_position + state.num_grubbed_messages + 1;
                    state.num_total_messages = approximate_total.max(state.num_grubbed_messages);
                } else {
                    warn!(
                        "Channel_subsequent grubber failed to calculate approximate total messages id={}",
                        id
                    );
                }
                tx.send(GrubberMessage::StateUpdate(state.clone()))
                    .await
                    .unwrap();
                if message_pointer.id == message_end.id {
                    state.num_total_messages = state.num_grubbed_messages;
                    state.is_completed = true;
                    message_tx.send(MessageBulk::End).await.unwrap();
                    tx.send(GrubberMessage::StateUpdate(state.clone()))
                        .await
                        .unwrap();
                    break;
                }
                let mut bulk = channel
                    .messages(ctx, GetMessages::new().after(message_pointer.id))
                    .await
                    .unwrap();
                bulk.sort_by(|a, b| a.id.partial_cmp(&b.id).unwrap());
                let iter = bulk.iter().take_while(|mes| mes.id <= message_end.id);
                message_pointer = iter.clone().last().unwrap().clone();
                state.num_grubbed_messages += iter.clone().count() as u64;
                info!(
                    "Grubbed {} messages in channel_subsequent id={}",
                    state.num_grubbed_messages, id
                );

                tx.send(GrubberMessage::StateUpdate(state.clone()))
                    .await
                    .unwrap();

                for bulk in iter.map(|mes| MessageBulk::Continue(mes.clone())) {
                    message_tx.send(bulk).await.unwrap();
                }
            }
            info!("Channel_subsequent grubber finished id={}", id);
        });
        Ok(())
    }
}
async fn retrieve_final_message(
    ctx: &'_ serenity::Context,
    channel: &GuildChannel,
) -> Result<Message, Error> {
    let message = channel
        .messages(
            ctx,
            GetMessages::new()
                .before(
                    channel
                        .last_message_id
                        .expect("The channel is not text channel."),
                )
                .limit(1),
        )
        .await?;
    Ok(message
        .first()
        .expect("The channel has no messages.")
        .clone())
}
