use std::time::Duration;

use anyhow::anyhow;
use futures_util::stream::StreamExt;
use lib::surreal::{chat::get_or_create_chat, EventMessage, SurrealEvent};
use rdkafka::producer::{FutureProducer, FutureRecord};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use surrealdb::Notification;
use tokio::sync::mpsc::UnboundedSender;
use warp::filters::ws::Message;

use lib::client::SurrealDB;

pub async fn spawn_task(
    db: SurrealDB,
    producer: FutureProducer,
    msg: &str,
    tx: UnboundedSender<Message>,
) -> anyhow::Result<()> {
    let event = serde_json::from_str::<EventMessage>(msg)?;

    match producer
        .send(
            FutureRecord::to("commands").key("commands").payload(msg),
            Duration::from_secs(1000),
        )
        .await
    {
        Ok(delivery) => tracing::info!("kafka message sent: {:?}", delivery),
        Err((e, _)) => return Err(anyhow!("kafka error: {:?}", e)),
    }

    tokio::spawn(async move {
        if let Err(e) = handle_event(&event, db, tx).await {
            tracing::error!("handle_event error: {:?}", e);
        }
    });
    Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
struct EventNotification {
    event: SurrealEvent,
    username: String,
}

pub async fn handle_event(
    event: &EventMessage,
    db: SurrealDB,
    tx: UnboundedSender<Message>,
) -> anyhow::Result<()> {
    tracing::debug!("handle_event: {:?}", &event);
    match event {
        EventMessage::JoinChat(payload) => {
            let chat_thing = payload.data.chat.to_owned();
            let chat = get_or_create_chat(&db, &chat_thing).await?;

            if let None = chat {
                return Err(anyhow!("task: couldn't get or create chat.."));
            }

            let sql = format!(
                "LIVE SELECT out.* as event, out.data.user.username as username FROM chat_events WHERE in = {};",
                &chat_thing.to_raw()
            );

            let mut response = db.client.query(sql).await?;

            tracing::debug!("LIVE SELECT: {:?}", &response);

            let mut stream = response.stream::<Notification<EventNotification>>(0)?;
            while let Some(result) = stream.next().await {
                tracing::debug!("notification: {:?}", &result);
                let notification: EventNotification = result?.data;
                let event_message: EventMessage = notification.event.into();

                let message: Option<Value> = match &event_message {
                    EventMessage::ChatJoined(m) => Some(json!({
                        "type": "chatJoined",
                        "created_at": m.created_at,
                        "username": notification.username,
                    })),
                    EventMessage::ChatMessageSent(m) => Some(json!({
                        "type": "chatMessageSent",
                        "created_at": m.created_at,
                        "username": notification.username,
                        "text": m.data.text,
                    })),
                    _ => {
                        tracing::trace!("unhandled event: {:?}", event_message);
                        None
                    }
                };

                match message {
                    Some(message) => tx.send(Message::text(message.to_string()))?,
                    None => tracing::debug!("no message to send for event: {:?}", event_message),
                }
            }
        }
        _ => tracing::trace!("unhandled event: {:?}", event),
    }
    Ok(())
}
