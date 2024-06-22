use anyhow::Result;
use lib::{
    client::SurrealDB,
    surreal::{chat::models::ChatMessage, EventMessage},
};

use crate::process::ProcessEvent;

impl ProcessEvent for &ChatMessage {
    async fn process(&self, r#type: &str, db: SurrealDB) -> Result<Option<EventMessage>> {
        tracing::debug!("processing event for: {:?}", &self);
        let user_thing = &self.data.user;
        let chat_thing = &self.data.chat;
        let text = &self.data.text;

        let response = db
            .client
            .query("BEGIN TRANSACTION;")
            .query("LET $event = CREATE event SET type = $type, created_at = time::now(), data = { chat: $chat, user: $user, text: $text };")
            .query("RELATE $chat->chat_events->$event;")
            .query("COMMIT TRANSACTION;")
            .bind(("type", r#type))
            .bind(("user", user_thing))
            .bind(("chat", chat_thing))
            .bind(("text", text))
            .await?;
        tracing::debug!("processing response: {:?}", &response);
        Ok(None)
    }
}
