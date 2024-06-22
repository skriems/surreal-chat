use anyhow::Result;
use lib::{client::SurrealDB, surreal::EventMessage};

use crate::process::ProcessEvent;

impl ProcessEvent for EventMessage {
    async fn process(&self, r#_type: &str, db: SurrealDB) -> Result<Option<EventMessage>> {
        match self {
            EventMessage::JoinChat(payload) => payload.process("chatJoined", db).await,
            EventMessage::SendChatMessage(payload) => payload.process("chatMessageSent", db).await,
            _ => {
                tracing::warn!("ProcessEvent not implemented for {:?}", self);
                Ok(None)
            }
        }
    }
}
