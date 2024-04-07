use anyhow::Result;
use poise::serenity_prelude::{ChannelId, Message};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum ThreaderConfiguration {
    AnotherChannel { channel_id: Option<ChannelId> },
    NewThread {},
    NewForumPost {},
}
impl Default for ThreaderConfiguration {
    fn default() -> Self {
        Self::AnotherChannel { channel_id: None }
    }
}

pub trait Threader {
    async fn thread(
        &self,
        id: &str,
        tx: mpsc::Sender<ThreaderMessage>,
        rx: mpsc::Receiver<MessageBulk>,
    ) -> Result<()>;
}

pub enum ThreaderMessage {
    StateUpdate(ThreaderState),
}

pub struct ThreaderState {
    pub(crate) num_threaded_messages: u64,
    pub(crate) is_completed: bool,
}

pub enum MessageBulk {
    Continue(Message),
    End,
}
