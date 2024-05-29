use crate::serde::string_thing;
/// This module contains database models which are used
/// to interact with the database directly which makes
/// serialization and deserialization easier.
///
/// When communicating with clients we use structs from
/// the messages module.
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use super::db::{DBChat, DBChatMessage, DBChatMessageData, DBCreateChatMessage, DBJoinChat, DBJoinChatData};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateChat {
    pub events: Vec<Thing>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Chat {
    #[serde(with = "string_thing")]
    pub id: Thing,
    pub events: Vec<Thing>,
}

impl From<DBChat> for Chat {
    fn from(data: DBChat) -> Self {
        Self {
            id: data.id,
            events: data.events,
        }
    }
}

///

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChatMessage {
    #[serde(with = "string_thing")]
    pub user: Thing,
    pub data: ChatMessageData,
}

impl From<DBChatMessage> for ChatMessage {
    fn from(data: DBChatMessage) -> Self {
        Self {
            user: data.user,
            data: data.data.into(),
        }
    }
}

impl From<DBCreateChatMessage> for ChatMessage {
    fn from(data: DBCreateChatMessage) -> Self {
        Self {
            user: data.user,
            data: data.data.into(),
        }
    }
}

///

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChatMessageData {
    #[serde(with = "string_thing")]
    pub chat: Thing,
    pub username: String,
    pub text: String,
}

impl From<DBChatMessageData> for ChatMessageData {
    fn from(data: DBChatMessageData) -> Self {
        Self {
            chat: data.chat,
            username: data.username,
            text: data.text,
        }
    }
}

///

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct JoinChat {
    #[serde(with = "string_thing")]
    pub user: Thing,
    pub data: JoinChatData,
}

impl From<DBJoinChat> for JoinChat {
    fn from(data: DBJoinChat) -> Self {
        Self {
            user: data.user,
            data: data.data.into(),
        }
    }
}

///

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JoinChatData {
    #[serde(with = "string_thing")]
    pub chat: Thing,
    pub username: String,
}

impl From<DBJoinChatData> for JoinChatData {
    fn from(data: DBJoinChatData) -> Self {
        Self { chat: data.chat, username: data.username}
    }
}
