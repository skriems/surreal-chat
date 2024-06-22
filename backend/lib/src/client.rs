use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::{self, Client};
use surrealdb::opt::auth::Root;
use surrealdb::sql::Thing;
use surrealdb::{Result, Surreal};

use crate::surreal::EventMessage;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Record {
    #[allow(dead_code)]
    pub id: Thing,
}

#[derive(Clone, Debug)]
pub struct SurrealDB {
    pub client: Surreal<Client>,
}

impl SurrealDB {
    pub async fn new() -> Result<Self> {
        let client = Surreal::new::<ws::Ws>("surrealdb:8000/rpc").await?;
        client
            .signin(Root {
                username: "admin",
                password: "admin",
            })
            .await?;
        client.use_ns("test").use_db("test").await?;
        Ok(Self { client })
    }

    pub async fn create_event(&self, event: EventMessage) -> Result<Option<Record>> {
        let records: Vec<Record> = self.client.create("event").content(event).await?;
        if let Some(record) = records.first() {
            tracing::debug!("created event: {:?}", &record);
            return Ok(Some(record.to_owned()));
        }
        Ok(None)
    }
}
