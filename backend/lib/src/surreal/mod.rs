pub mod chat;
pub mod user;

// use anyhow::Result;
use chat::models::{ChatMessage, DBChatMessage};
use serde::{Deserialize, Serialize};
use user::models::{DBUserMessage, UserMessage};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum EventMessage {
    ChangeUser(UserMessage),
    UserChanged(UserMessage),

    JoinChat(ChatMessage),
    ChatJoined(ChatMessage),

    SendChatMessage(ChatMessage),
    ChatMessageSent(ChatMessage),
}

impl From<EventMessage> for SurrealEvent {
    fn from(event: EventMessage) -> Self {
        match event {
            EventMessage::ChangeUser(data) => SurrealEvent::ChangeUser(data.into()),
            EventMessage::UserChanged(data) => SurrealEvent::UserChanged(data.into()),

            EventMessage::JoinChat(data) => SurrealEvent::JoinChat(data.into()),
            EventMessage::ChatJoined(data) => SurrealEvent::ChatJoined(data.into()),

            EventMessage::SendChatMessage(data) => SurrealEvent::SendChatMessage(data.into()),
            EventMessage::ChatMessageSent(data) => SurrealEvent::ChatMessageSent(data.into()),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum SurrealEvent {
    ChangeUser(DBUserMessage),
    UserChanged(DBUserMessage),

    JoinChat(DBChatMessage),
    ChatJoined(DBChatMessage),

    SendChatMessage(DBChatMessage),
    ChatMessageSent(DBChatMessage),
}

impl From<SurrealEvent> for EventMessage {
    fn from(event: SurrealEvent) -> Self {
        match event {
            SurrealEvent::ChangeUser(data) => EventMessage::ChangeUser(data.into()),
            SurrealEvent::UserChanged(data) => EventMessage::UserChanged(data.into()),

            SurrealEvent::JoinChat(data) => EventMessage::JoinChat(data.into()),
            SurrealEvent::ChatJoined(data) => EventMessage::ChatJoined(data.into()),

            SurrealEvent::SendChatMessage(data) => EventMessage::SendChatMessage(data.into()),
            SurrealEvent::ChatMessageSent(data) => EventMessage::ChatMessageSent(data.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use surrealdb::sql::Thing;
    use user::models::UserMessageData;

    use super::*;

    #[test]
    fn test_change_user_into_surreal_event() {

        let data = UserMessageData {
            id: Thing::from_str("user:test").unwrap(),
            username: "test".to_string(),
        };
        let message = EventMessage::ChangeUser(UserMessage {
            created_at: None,
            data
        });

        let event: SurrealEvent = message.into();

        match event {
            SurrealEvent::ChangeUser(data) => {
                assert_eq!(data.data.id, Thing::from_str("user:test").unwrap());
                assert_eq!(data.data.username, "test");
            }
            _ => panic!("unexpected event: {:?}", event),
        }
    }
    // TODO: Add more tests
}
