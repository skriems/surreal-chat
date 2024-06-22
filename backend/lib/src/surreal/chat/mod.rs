use serde::{Deserialize, Serialize};
use surrealdb::{sql::Thing, Result};

use crate::client::SurrealDB;

pub mod models;

#[derive(Clone, Deserialize, Serialize)]
pub struct Chat {
    id: Thing,
}

pub async fn create_chat(db: &SurrealDB, thing: Option<&Thing>) -> Result<Option<Chat>> {
    if let Some(thing) = thing {
        let chat: Option<Chat> = db
            .client
            .create(thing)
            .await?;
        Ok(chat)
    } else {
        let chats: Vec<Chat> = db
            .client
            .create("chat")
            .await?;
        if let Some(chat) = chats.first() {
            Ok(Some(chat.to_owned()))
        } else {
            Ok(None)
        }
    }
}

pub async fn get_chat(db: &SurrealDB, id: &Thing) -> Result<Option<Chat>> {
    db.client.select(id).await
}

pub async fn get_or_create_chat(db: &SurrealDB, id: &Thing) -> Result<Option<Chat>> {
    if let Some(chat) = get_chat(db, id).await? {
        Ok(Some(chat))
    } else {
        create_chat(db, Some(id)).await
    }
}
