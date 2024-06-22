use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Thing};

use crate::serde::string_thing;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChatMessage {
    pub data: ChatMessageData,
    pub created_at: Option<Datetime>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChatMessageData {
    #[serde(with = "string_thing")]
    pub user: Thing,
    #[serde(with = "string_thing")]
    pub chat: Thing,
    pub text: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DBChatMessage {
    data: DBChatMessageData,
    created_at: Option<Datetime>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DBChatMessageData {
    pub user: Thing,
    pub chat: Thing,
    pub text: Option<String>,
}

impl From<DBChatMessage> for ChatMessage {
    fn from(data: DBChatMessage) -> Self {
        Self {
            created_at: data.created_at,
            data: data.data.into(),
        }
    }
}

impl From<ChatMessage> for DBChatMessage {
    fn from(data: ChatMessage) -> Self {
        Self {
            created_at: data.created_at,
            data: data.data.into(),
        }
    }
}

impl From<DBChatMessageData> for ChatMessageData {
    fn from(data: DBChatMessageData) -> Self {
        Self {
            user: data.user,
            chat: data.chat,
            text: data.text,
        }
    }
}

impl From<ChatMessageData> for DBChatMessageData {
    fn from(data: ChatMessageData) -> Self {
        Self {
            user: data.user,
            chat: data.chat,
            text: data.text,
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use std::str::FromStr;
//
//     use super::*;
//     use surrealdb::sql::Thing;
//
//     #[test]
//     fn test_into_db_chat_message_data() {
//         let data = ChatMessageData {
//             chat: Thing::from_str("chat:xyz").expect("should work"),
//             username: "john".to_string(),
//             text: "test".to_string(),
//         };
//
//         let thing = Thing::from_str("chat:xyz").expect("could not j");
//         assert_eq!(&data.chat.tb, &thing.tb);
//
//         let other: DBChatMessageData = data.into();
//         assert_eq!(other.chat, thing );
//     }
//
//     #[test]
//     fn test_into_db_chat_message() {
//         let msg = ChatMessage {
//             user: Thing::from_str("user:xyz").expect("should work"),
//             data: ChatMessageData {
//                 chat: Thing::from_str("chat:xyz").expect("should work"),
//                 username: "john".to_string(),
//                 text: "test".to_string(),
//             },
//         };
//
//         let other: DBChatMessage = msg.into();
//         assert_eq!(other.user, Thing::from_str("user:xyz").expect("should work") );
//     }
// }
