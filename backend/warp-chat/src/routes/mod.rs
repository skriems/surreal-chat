mod events;
mod ws;

use std::{collections::HashMap, sync::Arc};

use events::events;
use lib::client::SurrealDB;
use tokio::sync::{mpsc, RwLock};
use warp::ws::Message;
use warp::Filter;
use ws::websocket;

/// Our state of currently connected users.
/// - Key is their id
/// - Value is a sender of `warp::ws::Message`
pub type Users = Arc<RwLock<HashMap<String, mpsc::UnboundedSender<Message>>>>;
pub type Subscriptions = Arc<RwLock<HashMap<String, Vec<String>>>>;

pub fn with_db(
    db: SurrealDB,
) -> impl Filter<Extract = (SurrealDB,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

pub fn get_routes(
    db: SurrealDB,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let users = Users::default();
    let subscriptions = Subscriptions::default();

    let index = warp::path::end().map(|| warp::reply::html(std::include_str!("../../chat.html")));
    index
        .or(websocket(db.clone(), users, subscriptions))
        .or(events(db))
        .with(warp::log("warp"))
}
