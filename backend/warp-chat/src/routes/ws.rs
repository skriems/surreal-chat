use lib::client::SurrealDB;
use warp::Filter;

use super::{with_db, Subscriptions, Users};
use crate::handler::ws::on_upgrade;

pub fn websocket(
    db: SurrealDB,
    users: Users,
    subscriptions: Subscriptions,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    let user_filter = warp::any().map(move || users.clone());
    let subscriptions_filter = warp::any().map(move || subscriptions.clone());
    warp::path("ws")
        // The `ws()` filter will prepare Websocket handshake...
        .and(warp::ws())
        .and(user_filter)
        .and(subscriptions_filter)
        .and(with_db(db))
        .map(|ws: warp::ws::Ws, users, subscriptions, db: SurrealDB| {
            // This will call our function if the handshake succeeds.
            ws.on_upgrade(move |socket| on_upgrade(socket, users, subscriptions, db))
        })
}
