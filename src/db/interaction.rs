use anyhow::Result;
use bson::oid::ObjectId;
use bson::{doc, DateTime};
use serde::{Deserialize, Serialize};
use tokio::task;

use crate::framework::interactions::PendingInteraction;

use super::MongoDB;

#[derive(Debug, Serialize, Deserialize)]
pub struct PendingInteractionDoc {
    #[serde(skip_serializing)]
    pub _id: Option<ObjectId>,
    pub interaction: PendingInteraction,
    pub created_at: DateTime,
}

impl MongoDB {
    /// インタラクションをデータベースから取得します。
    /// `delete_after` が `true` の場合、取得後にデータベースから削除します。
    pub async fn interaction_find(
        &self,
        id: &ObjectId,
        delete_after: bool,
    ) -> Result<Option<PendingInteraction>> {
        let filter = doc! { "_id": id };
        let pending_interaction = self
            .interactions
            .find_one(filter.clone(), None)
            .await?
            .map(|doc| doc.interaction);
        let interactions = self.interactions.clone();

        if delete_after && pending_interaction.is_some() {
            task::spawn(async move { interactions.delete_one(filter, None).await });
        }

        Ok(pending_interaction)
    }

    /// インタラクションをデータベースに挿入します。
    pub async fn interaction_insert(&self, interaction: PendingInteraction) -> Result<ObjectId> {
        let interactions = &self.interactions;
        let doc = PendingInteractionDoc {
            _id: None,
            interaction,
            created_at: DateTime::now(),
        };
        let result = interactions.insert_one(doc, None).await?;
        Ok(result.inserted_id.as_object_id().unwrap())
    }
}
