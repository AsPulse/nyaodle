use bson::oid::ObjectId;
use bson::DateTime;
use serde::{Deserialize, Serialize};

use crate::threader::ThreaderConfiguration;

use super::MongoDB;

#[derive(Debug, Serialize, Deserialize)]
pub struct ThreaderConfigurationDoc {
    #[serde(skip_serializing)]
    pub _id: Option<ObjectId>,
    pub configuration: ThreaderConfiguration,
    pub created_at: DateTime,
}

impl MongoDB {
    pub async fn threader_configurations(&self) -> mongodb::Collection<ThreaderConfigurationDoc> {
        self.db
            .collection::<ThreaderConfigurationDoc>("threader_configurations")
    }
}
