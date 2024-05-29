// #![deny(warnings)]
use crate::events::spawn_action_task;
use crate::routes::{Subscriptions, Users};
use futures_util::{SinkExt, StreamExt, TryFutureExt};
use lib::actions::Action;
use lib::client::SurrealDB;
use lib::users::db::DBUser;
use lib::users::messages::{User, UserChanged};
use lib::users::{create_user, get_user};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use tracing;
use warp::ws::{Message, WebSocket};

pub async fn on_upgrade(ws: WebSocket, users: Users, _subscriptions: Subscriptions, db: SurrealDB) {
    // Split the socket into a sender and receiver of messages.
    let (mut sender, mut receiver) = ws.split();

    // 1st thing after upgrade, get or create the user in the db
    let mut user: Option<DBUser> = None;
    while let Some(Ok(message)) = receiver.next().await {
        if let Ok(msg) = message.to_str() {
            tracing::debug!("user connected: {:?}", &msg);

            if let Ok(Some(usr)) = get_user(&db, msg).await {
                user = Some(usr);
            }
            if user.is_none() {
                if let Ok(Some(usr)) = create_user(&db, msg).await {
                    user = Some(usr);
                }
                tracing::debug!("created user {:?}", &user);
            }

            if let Some(user) = &user {
                let action = Action::UserChanged(UserChanged {
                    user: user.id.to_owned(),
                    data: User {
                        id: user.id.to_owned(),
                        username: user.username.to_owned(),
                    },
                });
                if let Ok(json) = serde_json::to_string(&action) {
                    let _ = sender.send(Message::text(json)).await;
                }
                break;
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
        // and sends a message back to the user
        tokio::task::spawn(async move {
            while let Some(message) = rx.next().await {
                sender
                    .send(message)
                    .unwrap_or_else(|e| {
                        eprintln!("websocket send error: {}", e);
                    })
                    .await;
            }
        });

        // every time the user sends a message spawn a task to handle it
        while let Some(result) = receiver.next().await {
            let msg = match result {
                Ok(msg) => msg,
                Err(e) => {
                    eprintln!("websocket error(uid={}): {}", &user.username, e);
                    break;
                }
            };

            tracing::debug!("received message: {:?}", &msg);

            if let Ok(msg) = msg.to_str() {
                if let Err(error) = spawn_action_task(db.clone(), msg, tx.clone()).await {
                    tracing::error!("handle_event error: {:?}", &error);
                }
            }
        }
        // receiver stream will keep processing as long as the user stays
        // connected. Once they disconnect, then...
        user_disconnected(&user, &users).await;
    }
}

async fn user_message(user: &DBUser, msg: Message, users: &Users) {
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

async fn user_disconnected(user: &DBUser, users: &Users) {
    tracing::info!("good bye: {}", &user.username);

    let msg = format!("{} left.", &user.username);
    user_message(&user, Message::text(msg), &users).await;

    // Stream closed up, so remove from the user list
    users.write().await.remove(&user.username);
}
