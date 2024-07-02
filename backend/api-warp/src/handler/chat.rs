use anyhow::Result;
use lib::{client::SurrealDB, surreal::EventMessage};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;

// The query parameters for list_todos.
// #[derive(Debug, Deserialize)]
// pub struct ListOptions {
//     pub offset: Option<usize>,
//     pub limit: Option<usize>,
// }

#[derive(Clone, Debug, Deserialize, Serialize)]
struct ChatEvent {
    event: EventMessage,
    username: String,
}

pub async fn get_chat(id: String, db: SurrealDB) -> Result<impl warp::Reply, Infallible> {
    let query = format!("SELECT out[*] AS event, out.data.user.username AS username FROM chat_events WHERE in = chat:{};", id);
    let res = db.client.query(query).await;
    tracing::debug!("get_chat.result: {:?}", &res);

    if let Ok(mut response) = res {
        let results: surrealdb::Result<Vec<ChatEvent>> = response.take(0);
        match results {
            Ok(events) => return Ok(warp::reply::json(&events)),
            Err(e) => {
                tracing::error!("get_chat.error: {:?}", &e);
                let v: Vec<ChatEvent> = vec![];
                return Ok(warp::reply::json(&v))
            }
        }
    }
    let v: Vec<ChatEvent> = vec![];
    Ok(warp::reply::json(&v))
}
