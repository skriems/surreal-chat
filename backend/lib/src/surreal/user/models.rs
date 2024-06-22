use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Thing};

use crate::serde::string_thing;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserMessage {
    pub data: UserMessageData,
    pub created_at: Option<Datetime>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserMessageData {
    #[serde(with = "string_thing")]
    pub id: Thing,
    pub username: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DBUserMessage {
    pub data: DBUserMessageData,
    pub created_at: Option<Datetime>,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DBUserMessageData {
    pub id: Thing,
    pub username: String,
}

impl From<DBUserMessageData> for UserMessageData {
    fn from(data: DBUserMessageData) -> Self {
        Self {
            id: data.id,
            username: data.username,
        }
    }
}

impl From<UserMessageData> for DBUserMessageData {
    fn from(data: UserMessageData) -> Self {
        Self {
            id: data.id,
            username: data.username,
        }
    }
}

impl From<DBUserMessage> for UserMessage {
    fn from(data: DBUserMessage) -> Self {
        Self {
            created_at: data.created_at,
            data: data.data.into(),
        }
    }
}

impl From<UserMessage> for DBUserMessage {
    fn from(data: UserMessage) -> Self {
        Self {
            created_at: data.created_at,
            data: data.data.into(),
        }
    }
}
