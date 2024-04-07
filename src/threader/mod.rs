use poise::serenity_prelude::ChannelId;
use serde::{Deserialize, Serialize};

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
