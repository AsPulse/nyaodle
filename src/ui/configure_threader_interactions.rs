use anyhow::Result;
use bson::oid::ObjectId;
use bson::DateTime;

use crate::db::interaction::PendingInteractionDoc;
use crate::db::threader_configurations::ThreaderConfigurationDoc;
use crate::db::MongoDBExt;
use crate::framework::interactions::PendingInteraction;
use crate::threader::ThreaderConfiguration;
use crate::ApplicationContext;

pub struct ConfigureThreaderDocs {
    pub config: ThreaderConfiguration,
    pub select_id: ObjectId,
    pub execute_id: ObjectId,
    pub close_id: ObjectId,
}

impl ConfigureThreaderDocs {
    pub async fn create_and_insert(ctx: &ApplicationContext<'_>) -> Result<Self> {
        let mongo = ctx.mongo();
        let config = ThreaderConfiguration::default();
        let config_doc = mongo
            .threader_configurations()
            .await
            .insert_one(
                ThreaderConfigurationDoc {
                    _id: None,
                    configuration: config.clone(),
                    created_at: DateTime::now(),
                },
                None,
            )
            .await?;
        let id = config_doc.inserted_id.as_object_id().unwrap();
        let ids = mongo
            .interactions()
            .await
            .insert_many(
                [
                    PendingInteractionDoc {
                        _id: None,
                        interaction: PendingInteraction::SelectThreaders { config_id: id },
                        created_at: DateTime::now(),
                    },
                    PendingInteractionDoc {
                        _id: None,
                        interaction: PendingInteraction::ExecuteNyaodle { config_id: id },
                        created_at: DateTime::now(),
                    },
                    PendingInteractionDoc {
                        _id: None,
                        interaction: PendingInteraction::CloseThreadersConfig { config_id: id },
                        created_at: DateTime::now(),
                    },
                ],
                None,
            )
            .await?;
        Ok(Self {
            config,
            select_id: ids.inserted_ids.get(&0).unwrap().as_object_id().unwrap(),
            execute_id: ids.inserted_ids.get(&1).unwrap().as_object_id().unwrap(),
            close_id: ids.inserted_ids.get(&2).unwrap().as_object_id().unwrap(),
        })
    }
}
