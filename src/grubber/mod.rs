//! 移動させるメッセージを取得する

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

use crate::threader::MessageBulk;

use self::channel_subsequent::ChannelSubsequent;

pub mod channel_subsequent;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum NyaodleRequest {
    ChannelSubsequent(ChannelSubsequent),
}

pub trait Grubber {
    async fn grub(&self, id: &str, tx: mpsc::Sender<GrubberMessage>) -> Result<()>;
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
