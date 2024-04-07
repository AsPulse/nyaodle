//! 移動させるメッセージを取得する

use poise::serenity_prelude as serenity;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

use crate::threader::MessageBulk;
use crate::Error;

use self::channel_subsequent::{ChannelSubsequent, ChannelSubsequentGrubber};

pub mod channel_subsequent;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum NyaodleRequest {
    ChannelSubsequent(ChannelSubsequent),
}

impl NyaodleRequest {
    pub fn create_grabber<'a>(&self, ctx: &'a serenity::Context) -> impl Grubber + 'a {
        match self {
            NyaodleRequest::ChannelSubsequent(property) => ChannelSubsequentGrubber {
                ctx,
                property: property.clone(),
            },
        }
    }
}

pub trait Grubber {
    fn grub(
        &self,
        id: &str,
        tx: mpsc::Sender<GrubberMessage>,
    ) -> impl std::future::Future<Output = Result<(), Error>> + std::marker::Send;
}

pub enum GrubberMessage {
    StateUpdate(GrubberState),
    MessageTranfer(Vec<MessageBulk>),
}

#[derive(Debug, Clone)]
pub struct GrubberState {
    pub(crate) num_total_messages: u64,
    pub(crate) num_grubbed_messages: u64,
    pub(crate) is_completed: bool,
}
