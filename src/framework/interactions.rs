use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum PendingInteraction {
    SelectThreaders { config_id: ObjectId },
    ExecuteNyaodle { config_id: ObjectId },
    CloseThreadersConfig { config_id: ObjectId },
}
