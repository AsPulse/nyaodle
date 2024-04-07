use crate::ApplicationContext;

use super::MongoDBExt;

use anyhow::Result;
use log::{info, warn};
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, DateTime};

use serde::{Deserialize, Serialize};

use mongodb::options::UpdateOptions;
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RaceKind {
    CommandInvocation,
    ReceiveInteraction,
}

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct WsRace {
    _id: Option<ObjectId>,
    kind: RaceKind,
    token: String,
    acquired: u32,
    created_at: DateTime,
}

pub async fn race_interaction(ctx: &ApplicationContext<'_>) -> Result<bool> {
    race(
        ctx,
        RaceKind::ReceiveInteraction,
        ctx.interaction.id.to_string(),
    )
    .await
}
pub async fn race(ctx: &impl MongoDBExt, kind: RaceKind, token: String) -> Result<bool> {
    let mongo = &ctx.mongo();
    let race = mongo
        .ws_race()
        .await
        .update_one(
            doc! { "kind": bson::to_bson(&kind).unwrap(), "token": &token },
            doc! {
                "$setOnInsert" : { "created_at": DateTime::now() },
                "$inc": {
                    "acquired": 1
                }
            },
            UpdateOptions::builder().upsert(true).build(),
        )
        .await;
    let Ok(race) = race else {
        warn!(
            "Skipped: {:?} ({:?}) with error: {:?}",
            kind,
            token,
            race.unwrap_err()
        );
        return Ok(true);
    };
    if race.upserted_id.is_some() {
        info!("Acquired: {:?} ({:?})", kind, token);
        Ok(false)
    } else {
        info!("Skipped: {:?} ({:?})", kind, token);
        Ok(true)
    }
}
