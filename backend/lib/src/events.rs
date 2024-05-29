use serde::{Deserialize, Serialize};

use crate::{
    chats::{
        db::{DBCreateChatMessage, DBJoinChat},
        messages::CreateChat,
    },
    users::db::DBUserChanged,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Event {
    CreateChat(CreateChat),
    JoinChat(DBJoinChat),
    UserJoined(DBJoinChat),
    SendChatMessage(DBCreateChatMessage),
    ChatMessageSent(DBCreateChatMessage),
    UserChanged(DBUserChanged),
}
