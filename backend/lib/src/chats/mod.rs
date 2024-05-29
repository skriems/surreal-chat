pub mod db;
pub mod messages;

use crate::client::SurrealDB;
use db::DBChat;
use messages::CreateChat;
use surrealdb::{sql::Thing, Result};

pub async fn create_chat(db: &SurrealDB, thing: Option<&Thing>) -> Result<Option<DBChat>> {
    if let Some(thing) = thing {
        let chat: Option<DBChat> = db
            .client
            .create(thing)
            .content(CreateChat { events: vec![] })
            .await?;
        Ok(chat)
    } else {
        let chats: Vec<DBChat> = db
            .client
            .create("chat")
            .content(CreateChat { events: vec![] })
            .await?;
        if let Some(chat) = chats.first() {
            Ok(Some(chat.to_owned()))
        } else {
            Ok(None)
        }
    }
}

pub async fn get_chat(db: &SurrealDB, id: &Thing) -> Result<Option<DBChat>> {
    db.client.select(id).await
}

pub async fn get_or_create_chat(db: &SurrealDB, id: &Thing) -> Result<Option<DBChat>> {
    if let Some(chat) = get_chat(db, id).await? {
        Ok(Some(chat))
    } else {
        create_chat(db, Some(id)).await
    }
}
