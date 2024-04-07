pub mod interaction;
pub mod race;
pub mod threader_configurations;

use anyhow::Result;
use log::info;

use mongodb::bson::doc;

use mongodb::IndexModel;

use std::env;
use std::time::Duration;

use mongodb::options::{ClientOptions, IndexOptions};

use crate::{ApplicationContext, Context, Data};

use self::interaction::PendingInteractionDoc;
use self::race::WsRace;
use self::threader_configurations::ThreaderConfigurationDoc;

pub struct MongoDB {
    pub client: mongodb::Client,
    pub db: mongodb::Database,

    pub(self) ws_race: mongodb::Collection<WsRace>,
    pub threader_configurations: mongodb::Collection<ThreaderConfigurationDoc>,
    pub interactions: mongodb::Collection<PendingInteractionDoc>,
}

pub struct Collections {}

impl MongoDB {
    pub async fn connect() -> Result<MongoDB> {
        let uri = env::var("MONGO_URI").expect("Expected MONGO_URI in the environment");
        let db_name = env::var("MONGO_DB").expect("Expected MONGO_DB in the environment");

        let client_options = ClientOptions::parse(uri).await?;
        let client = mongodb::Client::with_options(client_options)?;
        let db = client.database(db_name.as_str());
        info!("Connected to MongoDB ({}).", db_name);

        let mongo = Self {
            client,
            ws_race: db.collection::<WsRace>("ws_race"),
            threader_configurations: db
                .collection::<ThreaderConfigurationDoc>("threader_configurations"),
            interactions: db.collection::<PendingInteractionDoc>("interactions"),
            db,
        };

        info!("Creating indexes...");
        mongo
            .ws_race
            .create_index(
                IndexModel::builder()
                    .keys(doc! { "kind": 1, "token": 1 })
                    .options(IndexOptions::builder().unique(true).build())
                    .build(),
                None,
            )
            .await?;
        mongo
            .ws_race
            .create_index(
                IndexModel::builder()
                    .keys(doc! { "created_at": 1 })
                    .options(
                        IndexOptions::builder()
                            .expire_after(Duration::from_secs(300))
                            .build(),
                    )
                    .build(),
                None,
            )
            .await?;
        mongo
            .interactions
            .create_index(
                IndexModel::builder()
                    .keys(doc! { "interaction.config_id": 1 })
                    .options(IndexOptions::builder().build())
                    .build(),
                None,
            )
            .await?;
        info!("Indexes created.");

        Ok(mongo)
    }
}
pub trait MongoDBExt {
    fn mongo(&self) -> &MongoDB;
}

impl MongoDBExt for Context<'_> {
    fn mongo(&self) -> &MongoDB {
        &self.data().mongo
    }
}

impl MongoDBExt for ApplicationContext<'_> {
    fn mongo(&self) -> &MongoDB {
        &self.data().mongo
    }
}

impl MongoDBExt for Data {
    fn mongo(&self) -> &MongoDB {
        &self.mongo
    }
}
