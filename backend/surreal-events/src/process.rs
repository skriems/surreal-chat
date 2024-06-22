use std::fmt::Debug;

use anyhow::Result;
use lib::{client::SurrealDB, surreal::EventMessage};

pub trait ProcessEvent {
    async fn process(&self, r#_type: &str, _db: SurrealDB) -> Result<Option<EventMessage>>
    where
        Self: Debug,
    {
        tracing::warn!("ProcessEvent trait not implemented for {:?}", self);
        Ok(None)
    }
}

pub async fn process<T>(event: T, r#type: &str, db: SurrealDB) -> Result<Option<EventMessage>>
where
    T: ProcessEvent + Debug,
{
    event.process(r#type, db).await
}
