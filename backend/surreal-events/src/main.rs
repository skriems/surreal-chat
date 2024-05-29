// #![deny(warnings)]
use anyhow::Result;
use futures_util::StreamExt;
use lib::chats::db::{DBChat, DBCreateChatMessage, DBJoinChat};
use lib::chats::get_chat;
use lib::chats::messages::CreateChat;
use surrealdb::engine::remote::ws::Client;
use surrealdb::method::Stream;
use tracing;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use lib::client::SurrealDB;
use lib::events::Event;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "surreal_events=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // connect to surrealdb
    let db = SurrealDB::new().await?;

    let mut stream: Stream<Client, Vec<Event>> = db.client.select("event").live().await?;

    while let Some(result) = stream.next().await {
        tracing::debug!("event notification: {:?}", &result);

        let event = result?.data;

        match event {
            Event::JoinChat(payload) => {
                let data = payload.data;
                let user = payload.user;
                if let Ok(Some(chat)) = get_chat(&db, &data.chat).await {
                    if let Ok(Some(record)) = db
                        .create_event(Event::UserJoined(DBJoinChat {
                            user,
                            data: data.clone(),
                        }))
                        .await
                    {
                        let mut events = chat.events;
                        events.push(record.id);
                        let _: Option<DBChat> = db
                            .client
                            .update(data.chat)
                            .merge(CreateChat { events })
                            .await?;
                    } else {
                        tracing::error!("couldn't create Event::UserJoined record..");
                    }
                } else {
                    tracing::error!("couldn't get {:?}", &data.chat.to_raw());
                }
            }
            Event::SendChatMessage(payload) => {
                let data = payload.data;
                let user = payload.user;
                if let Ok(Some(chat)) = get_chat(&db, &data.chat).await {
                    if let Ok(Some(record)) = db
                        .create_event(Event::ChatMessageSent(DBCreateChatMessage {
                            user,
                            data: data.clone(),
                        }))
                        .await
                    {
                        let mut events = chat.events;
                        events.push(record.id);
                        let _: Option<DBChat> = db
                            .client
                            .update(data.chat)
                            .merge(CreateChat { events })
                            .await?;
                    } else {
                        tracing::error!("couldn't create Event::ChatMessageSent record..");
                    }
                } else {
                    tracing::error!("couldn't get {:?}", &data.chat.to_raw());
                }
            }
            _ => tracing::trace!("unhandled event: {:?}", event),
        }
    }
    Ok(())
}
