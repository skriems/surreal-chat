/// This module contains database models which are used
/// to interact with the database directly which makes
/// serialization and deserialization easier.
///
/// When communicating with clients we use structs from
/// the messages module.
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

/// models chats when retrieved from the database
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DBChat {
    pub id: Thing,
    pub events: Vec<Thing>,
}

/// models an event when a user sends a chat message
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DBCreateChatMessage {
    pub user: Thing,
    pub data: DBChatMessageData,
}

/// models an event when a user sends a chat message
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DBChatMessage {
    pub id: Thing,
    pub user: Thing,
    pub data: DBChatMessageData,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DBChatMessageData {
    pub chat: Thing,
    pub username: String,
    pub text: String,
}

/// models an event when a user joins a chat
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DBJoinChat {
    pub user: Thing,
    pub data: DBJoinChatData,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DBJoinChatData {
    pub chat: Thing,
    pub username: String,
}
