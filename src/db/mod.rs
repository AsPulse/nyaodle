pub mod interaction;
pub mod race;
pub mod threader_configurations;

use anyhow::Result;
use log::info;

use mongodb::bson::doc;

use mongodb::IndexModel;

use std::env;

use mongodb::options::{ClientOptions, IndexOptions};

use crate::{ApplicationContext, Context, Data};

use self::race::WsRace;

pub struct MongoDB {
    pub client: mongodb::Client,
    pub db: mongodb::Database,
}

impl MongoDB {
    pub async fn connect() -> Result<MongoDB> {
        let uri = env::var("MONGO_URI").expect("Expected MONGO_URI in the environment");
        let db_name = env::var("MONGO_DB").expect("Expected MONGO_DB in the environment");

        let client_options = ClientOptions::parse(uri).await?;
        let client = mongodb::Client::with_options(client_options)?;
        let db = client.database(db_name.as_str());
        info!("Connected to MongoDB ({}).", db_name);

        let mongo = Self { client, db };

        info!("Creating indexes...");
        mongo
            .ws_race()
            .await
            .create_index(
                IndexModel::builder()
                    .keys(doc! { "kind": 1, "token": 1 })
                    .options(IndexOptions::builder().unique(true).build())
                    .build(),
                None,
            )
            .await?;
        info!("Indexes created.");

        Ok(mongo)
    }

    async fn ws_race(&self) -> mongodb::Collection<WsRace> {
        self.db.collection::<WsRace>("ws_race")
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
