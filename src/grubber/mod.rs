//! 移動させるメッセージを取得する

use serde::{Deserialize, Serialize};

pub mod channel_subsequent;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum NyaodleRequest {
    ChannelSubsequent { message_id: u64 },
}
