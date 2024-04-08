use bson::oid::ObjectId;
use bson::DateTime;
use serde::{Deserialize, Serialize};

use crate::grubber::NyaodleRequest;
use crate::threader::ThreaderConfiguration;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ThreaderConfigurationDoc {
    #[serde(skip_serializing)]
    pub _id: Option<ObjectId>,
    pub configuration: ThreaderConfiguration,
    pub request: NyaodleRequest,
    pub created_at: DateTime,
}
