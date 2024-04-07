use anyhow::Result;
use bson::oid::ObjectId;
use bson::{doc, Bson, DateTime};
use futures::StreamExt;
use log::error;
use serde::{Deserialize, Serialize};

use crate::db::interaction::PendingInteractionDoc;
use crate::db::threader_configurations::ThreaderConfigurationDoc;
use crate::db::MongoDBExt;
use crate::framework::interactions::PendingInteraction;
use crate::threader::ThreaderConfiguration;
use crate::ApplicationContext;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigureThreaderDocs {
    pub config: ThreaderConfigurationDoc,
    pub select_id: ObjectId,
    pub execute_id: ObjectId,
    pub close_id: ObjectId,
}

impl ConfigureThreaderDocs {
    pub async fn create_and_insert(ctx: &ApplicationContext<'_>) -> Result<Self> {
        let mongo = ctx.mongo();
        let config = ThreaderConfigurationDoc {
            _id: None,
            configuration: ThreaderConfiguration::default(),
            created_at: DateTime::now(),
        };
        let config_doc = mongo
            .threader_configurations
            .insert_one(config.clone(), None)
            .await?;
        let id = config_doc.inserted_id.as_object_id().unwrap();
        let ids = mongo
            .interactions
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
        let ids = [0, 1, 2]
            .iter()
            .map(|i| ids.inserted_ids.get(i).unwrap().as_object_id().unwrap())
            .collect::<Vec<ObjectId>>();
        Ok(Self {
            config,
            select_id: ids[0],
            execute_id: ids[1],
            close_id: ids[2],
        })
    }

    pub async fn from_interaction_ids(ctx: &impl MongoDBExt, id: ObjectId) -> Result<Self> {
        let mongo = ctx.mongo();
        let aggregate = mongo.interactions.aggregate(
            [
                doc! { "$match": { "_id": id } },
                doc! { "$lookup": {
                    "from": "threader_configurations",
                    "localField": "interaction.config_id",
                    "foreignField": "_id",
                    "as": "config"
                } },
                doc! { "$unwind": "$config" },
                doc! { "$lookup": {
                    "from": "interactions",
                    "localField": "config._id",
                    "foreignField": "interaction.config_id",
                    "as": "interactions"
                } },
                doc! { "$project": {
                    "config": 1,
                    "select_id": {
                        "$filter": {
                            "input": "$interactions",
                            "as": "v",
                            "cond": { "$eq": ["$$v.interaction.type", "select_threaders"] }
                        }
                    },
                    "execute_id": {
                        "$filter": {
                            "input": "$interactions",
                            "as": "v",
                            "cond": { "$eq": ["$$v.interaction.type", "execute_nyaodle"] }
                        }
                    },
                    "close_id": {
                        "$filter": {
                            "input": "$interactions",
                            "as": "v",
                            "cond": { "$eq": ["$$v.interaction.type", "close_threaders_config"] }
                        }
                    },
                } },
                doc! { "$project": {
                    "config": 1,
                    "select_id": { "$arrayElemAt": ["$select_id._id", 0] },
                    "execute_id": { "$arrayElemAt": ["$execute_id._id", 0] },
                    "close_id": { "$arrayElemAt": ["$close_id._id", 0] },
                } },
            ],
            None,
        );
        let Some(document) = aggregate.await?.next().await else {
            return Err(anyhow::anyhow!(
                "No document found for interaction id: {:?}",
                id
            ));
        };
        Ok(bson::from_document(document?)?)
    }
}
