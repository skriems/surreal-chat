// #![deny(warnings)]
mod events;
mod handler;
mod routes;

use std::net::SocketAddr;

use anyhow::Result;
use routes::get_routes;
use tracing;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use lib::client::SurrealDB;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "warp=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // connect to surrealdb
    let client = SurrealDB::new().await?;
    let routes = get_routes(client);
    let socket = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", &socket.to_string());
    warp::serve(routes).run(socket).await;
    Ok(())
}
