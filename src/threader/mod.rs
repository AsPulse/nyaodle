pub mod debug_threader;

use poise::serenity_prelude::{ChannelId, Message};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

use crate::Error;

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
    fn thread(
        &self,
        id: &str,
        tx: mpsc::Sender<ThreaderMessage>,
        rx: mpsc::Receiver<MessageBulk>,
    ) -> impl std::future::Future<Output = Result<(), Error>> + Send;
}

pub enum ThreaderMessage {
    StateUpdate(ThreaderState),
}

#[derive(Debug, Clone)]
pub struct ThreaderState {
    pub(crate) num_threaded_messages: u64,
    pub(crate) is_completed: bool,
}

pub enum MessageBulk {
    Continue(Message),
    End,
}
