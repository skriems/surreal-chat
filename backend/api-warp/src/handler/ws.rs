// #![deny(warnings)]
use futures_util::{SinkExt, StreamExt};
use lib::surreal::user::get_or_create_user;
use rdkafka::producer::FutureProducer;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use tracing;
use warp::ws::{Message, WebSocket};

use crate::events::spawn_task;
use crate::routes::{Subscriptions, Users};

use lib::client::SurrealDB;
use lib::surreal::user::models::{DBUserMessage, DBUserMessageData};
use lib::surreal::EventMessage;

pub async fn on_upgrade(
    ws: WebSocket,
    users: Users,
    _subscriptions: Subscriptions,
    db: SurrealDB,
    producer: FutureProducer,
) {
    // Split the socket into a sender and receiver of messages.
    let (mut sender, mut receiver) = ws.split();

    // 1st thing after upgrade, get or create the user in the db
    let mut user: Option<DBUserMessageData> = None;
    while let Some(Ok(message)) = receiver.next().await {
        if let Ok(msg) = message.to_str() {
            tracing::debug!("user connected: {:?}", &msg);
            match get_or_create_user(&db, msg).await {
                Ok(maybe_user) => {
                    tracing::debug!("user: {:?}", &maybe_user);
                    user = maybe_user;
                }
                Err(e) => {
                    tracing::warn!("get_or_create_user error: {:?}", &e);
                }
            }

            if let Some(user) = &user {
                let event = EventMessage::UserChanged(
                    DBUserMessage {
                        data: DBUserMessageData {
                            id: user.id.to_owned(),
                            username: user.username.to_owned(),
                        },
                        created_at: None,
                    }
                    .into(),
                );

                match serde_json::to_string(&event) {
                    Ok(json) => {
                        tracing::debug!("sending message: {:?}", &event);
                        let _ = sender.send(Message::text(json)).await;
                        break;
                    }
                    Err(e) => {
                        tracing::error!("serde_json::to_string error: {:?}", &e);
                    }
                }
            } else {
                let _ = sender.send(Message::text("Couldn't get user..")).await;
                return;
            }
        }
    }

    tracing::debug!("user: {:?}", &user);

    if let Some(user) = user {
        // Use an unbounded channel to handle buffering and flushing of messages
        // to the websocket...
        let (tx, rx) = mpsc::unbounded_channel();
        let mut rx = UnboundedReceiverStream::new(rx);
        // Save the sender in our list of connected users.
        users
            .write()
            .await
            .insert(user.username.to_owned(), tx.clone());

        // spawn a task that listens for internal messages from the unbounded channel
        // and sends a messages back to the user
        tokio::task::spawn(async move {
            while let Some(message) = rx.next().await {
                tracing::debug!("sending message: {:?}", &message);
                if let Err(e) = sender.send(message).await {
                    tracing::error!("websocket send error: {}", e);
                }
            }
        });

        // every time the user sends a message spawn a task to handle it
        while let Some(result) = receiver.next().await {
            tracing::debug!("received message: {:?}", &result);

            let msg = match result {
                Ok(msg) => msg,
                Err(e) => {
                    tracing::error!("websocket error(uid={}): {}", &user.username, e);
                    break;
                }
            };

            if let Ok(msg) = msg.to_str() {
                if let Err(error) = spawn_task(db.clone(), producer.clone(), msg, tx.clone()).await
                {
                    tracing::error!("spawn_action_task error: {:?}", &error);
                    tx.send(Message::text(format!(
                        "Error processing message: {:?}",
                        &error
                    )))
                    .unwrap_or_else(|e| {
                        tracing::error!("websocket send error: {}", e);
                    })
                }
            }
        }
        // receiver stream will keep processing as long as the user stays
        // connected. Once they disconnect, then...
        user_disconnected(&user, &users).await;
    }
}

async fn user_message(user: &DBUserMessageData, msg: Message, users: &Users) {
    // Skip any non-Text messages...
    let msg = if let Ok(s) = msg.to_str() {
        s
    } else {
        return;
    };

    // New message from this user
    for (_, tx) in users.read().await.iter() {
        if let Err(error) = tx.send(Message::text(msg)) {
            // The tx is disconnected, our `user_disconnected` code
            // should be happening in another task, nothing more to
            // do here.
            tracing::warn!(
                "Couldn't send message to {}. Probably disconnected...",
                &user.username
            );
            tracing::debug!("Error: {}", &error);
        }
    }
}

async fn user_disconnected(user: &DBUserMessageData, users: &Users) {
    tracing::info!("good bye: {}", &user.username);

    let msg = format!("{} left.", &user.username);
    user_message(&user, Message::text(msg), &users).await;

    // Stream closed up, so remove from the user list
    users.write().await.remove(&user.username);
}
