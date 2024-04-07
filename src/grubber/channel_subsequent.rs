//! あるチャンネルに投稿された、このメッセージ以降のメッセージを対象にする

use crate::db::race::race_interaction;
use crate::grubber::GrubberMessage;
use crate::grubber::GrubberState;
use crate::threader::MessageBulk;
use crate::ui::configure_threader::create::configure_threader;
use log::info;
use poise::command;
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::ChannelId;
use poise::serenity_prelude::GetMessages;
use poise::serenity_prelude::MessageId;
use serde::Deserialize;
use serde::Serialize;

use crate::ApplicationContext;
use crate::Error;

use super::Grubber;

#[command(context_menu_command = "これ以降のメッセージを移動")]
pub async fn move_channel_subsequent(
    ctx: ApplicationContext<'_>,
    _message: serenity::Message,
) -> Result<(), Error> {
    if race_interaction(&ctx).await? {
        return Ok(());
    }
    configure_threader(ctx).await?;
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
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
    ) -> anyhow::Result<()> {
        let ctx = self.ctx.clone();
        info!("New channel_subsequent grubber started with id={}", id);
        let Some(channel) = ctx
            .http
            .get_channel(self.property.channel_id)
            .await?
            .guild()
        else {
            return Err(anyhow::anyhow!("Specified channel is not a guild channel."));
        };
        let mut state = GrubberState {
            num_total_messages: 1,
            num_grubbed_messages: 1,
            is_completed: false,
        };
        let message_start = ctx
            .http
            .get_message(self.property.channel_id, self.property.message_id)
            .await?;
        let mut message_pointer = message_start.clone();
        let message_end = ctx
            .http
            .get_message(
                self.property.channel_id,
                channel
                    .last_message_id
                    .expect("Channel is not Text Channel."),
            )
            .await?;

        tx.send(GrubberMessage::MessageTranfer(vec![MessageBulk::Continue(
            message_pointer.clone(),
        )]))
        .await
        .unwrap();
        let id = id.to_string();
        tokio::spawn(async move {
            let ctx = &ctx;
            loop {
                let approximate_total = message_end.position.unwrap()
                    - message_pointer.position.unwrap()
                    + state.num_grubbed_messages
                    + 1;
                state.num_total_messages = approximate_total.max(state.num_grubbed_messages);
                tx.send(GrubberMessage::StateUpdate(state.clone()))
                    .await
                    .unwrap();
                if message_pointer.id == message_end.id {
                    state.num_total_messages = state.num_grubbed_messages;
                    state.is_completed = true;
                    tx.send(GrubberMessage::MessageTranfer(vec![MessageBulk::End]))
                        .await
                        .unwrap();
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
                tx.send(GrubberMessage::MessageTranfer(
                    iter.map(|mes| MessageBulk::Continue(mes.clone())).collect(),
                ))
                .await
                .unwrap();
            }
            info!("Channel_subsequent grubber finished id={}", id);
        });
        Ok(())
    }
}
