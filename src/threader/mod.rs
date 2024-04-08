pub mod another_channel;
pub mod debug_threader;

use log::{error, warn};
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::{ChannelId, Message};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

use crate::Error;

use self::another_channel::AnotherChannelThreader;
use self::debug_threader::DebugThreader;

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
impl ThreaderConfiguration {
    pub fn create_threader<'a>(&self, ctx: &'a serenity::Context) -> impl Threader + 'a {
        match self {
            ThreaderConfiguration::AnotherChannel { channel_id } => {
                let Some(channel_id) = channel_id else {
                    error!("ThreaderConfiguration::create_threader: channel_id is None");
                    return ThreaderVariant::Debug(debug_threader::DebugThreader { ctx });
                };
                ThreaderVariant::AnotherChannel(another_channel::AnotherChannelThreader {
                    ctx,
                    channel_id: *channel_id,
                })
            }
            _ => {
                warn!("ThreaderConfiguration::create_threader: not implemented");
                ThreaderVariant::Debug(debug_threader::DebugThreader { ctx })
            }
        }
    }
}

pub enum ThreaderVariant<'a> {
    AnotherChannel(AnotherChannelThreader<'a>),
    Debug(DebugThreader<'a>),
}

impl Threader for ThreaderVariant<'_> {
    async fn thread(
        &self,
        id: &str,
        tx: mpsc::Sender<ThreaderMessage>,
        rx: mpsc::Receiver<MessageBulk>,
    ) -> Result<(), Error> {
        match self {
            ThreaderVariant::AnotherChannel(threader) => threader.thread(id, tx, rx).await,
            ThreaderVariant::Debug(threader) => threader.thread(id, tx, rx).await,
        }
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
