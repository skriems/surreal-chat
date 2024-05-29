use anyhow::anyhow;
use futures_util::stream::StreamExt;
use surrealdb::{engine::remote::ws::Client, method::Stream};
use tokio::sync::mpsc::UnboundedSender;
use warp::filters::ws::Message;

use lib::actions::Action;
use lib::chats::{db::DBChat, get_or_create_chat};
use lib::{client::SurrealDB, events::Event};

pub async fn spawn_action_task(
    db: SurrealDB,
    msg: &str,
    tx: UnboundedSender<Message>,
) -> anyhow::Result<()> {
    let action = serde_json::from_str::<Action>(msg)?;
    tracing::debug!("spawning task to handle_action: {:?}", &action);
    tokio::spawn(async move {
        if let Err(e) = handle_action(db, &action, tx).await {
            tracing::error!("handle_action error: {:?}", e);
        }
    });
    Ok(())
}

pub async fn handle_action(
    db: SurrealDB,
    action: &Action,
    tx: UnboundedSender<Message>,
) -> anyhow::Result<()> {

    db.create_action(action.to_owned()).await?;

    match action {
        Action::JoinChat(payload) => {
            let chat_thing = payload.data.chat.to_owned();
            let chat = get_or_create_chat(&db, &chat_thing).await?;

            if let None = chat {
                return Err(anyhow!("task: couldn't get or create chat.."));
            }

            let mut stream: Stream<Client, Option<DBChat>> =
                db.client.select(chat_thing).live().await?;

            while let Some(result) = stream.next().await {
                tracing::debug!("task: notification: {:?}", &result);
                let chat = result?.data;
                if let Some(event_thing) = chat.events.last() {
                    let event_option: Option<Event> = db.client.select(event_thing).await?;
                    if let Some(event) = event_option {
                        if let Ok(json) = serde_json::to_string::<Action>(&event.into()) {
                            tx.send(Message::text(json))?;
                        }
                    }
                }
            }
        }
        _ => tracing::trace!("unhandled event: {:?}", action),
    }
    Ok(())
}
