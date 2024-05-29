use super::with_db;
use crate::handler::events::get_events;
use lib::client::SurrealDB;
use warp::Filter;

pub fn events(
    db: SurrealDB,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("events")
        .and(warp::get())
        // .and(warp::query::<ListOptions>())
        .and(with_db(db))
        .and_then(get_events)
}
