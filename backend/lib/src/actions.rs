use serde::{Deserialize, Serialize};

use crate::{
    chats::{
        db::{DBChatMessageData, DBCreateChatMessage, DBJoinChat, DBJoinChatData},
        messages::{ChatMessage, CreateChat, JoinChat},
    },
    events::Event,
    users::messages::UserChanged,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Action {
    CreateChat(CreateChat),
    JoinChat(JoinChat),
    UserJoined(JoinChat),
    SendChatMessage(ChatMessage),
    ChatMessageSent(ChatMessage),
    UserChanged(UserChanged),
}

impl From<Event> for Action {
    fn from(event: Event) -> Self {
        match event {
            Event::CreateChat(payload) => Action::CreateChat(payload),
            Event::JoinChat(payload) => Action::JoinChat(payload.into()),
            Event::UserJoined(payload) => Action::UserJoined(payload.into()),
            Event::SendChatMessage(payload) => Action::SendChatMessage(payload.into()),
            Event::ChatMessageSent(payload) => Action::ChatMessageSent(payload.into()),
            Event::UserChanged(payload) => Action::UserChanged(payload.into()),
        }
    }
}

pub fn create_event(action: Action) -> Option<Event> {
    match action {
        Action::CreateChat(payload) => Some(Event::CreateChat(payload)),
        Action::JoinChat(payload) => Some(Event::JoinChat(DBJoinChat {
            user: payload.user,
            data: DBJoinChatData {
                chat: payload.data.chat,
                username: payload.data.username,
            },
        })),
        Action::SendChatMessage(payload) => Some(Event::SendChatMessage(DBCreateChatMessage {
            user: payload.user,
            data: DBChatMessageData {
                chat: payload.data.chat,
                username: payload.data.username,
                text: payload.data.text,
            },
        })),
        _ => {
            tracing::debug!("unhandled action: {:?}", &action);
            None
        }
    }
}
