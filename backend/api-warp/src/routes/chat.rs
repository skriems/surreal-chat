use super::with_db;
use crate::handler::chat::get_chat;
use lib::client::SurrealDB;
use warp::Filter;

pub fn chat(
    db: SurrealDB,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("chat" / String)
        .and(with_db(db))
        // // .and(warp::query::<ListOptions>())
        .and_then(get_chat)
}
