// #![deny(warnings)]
use std::net::SocketAddr;

mod cli;
mod events;
mod handler;
mod routes;

use anyhow::Result;
use clap::ArgMatches;
use rdkafka::{producer::FutureProducer, ClientConfig};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use lib::client::SurrealDB;

use crate::routes::get_routes;

async fn run(matches: &ArgMatches) -> Result<()> {
    let brokers = matches
        .get_one::<String>("brokers")
        .expect("brokers is required");
    let domains = matches
        .get_many::<String>("domain")
        .unwrap_or_default()
        .map(|v| v.as_str())
        .collect::<Vec<_>>();
    let group_id = matches.get_one::<String>("group-id").unwrap();

    tracing::info!(
        "Starting kafka producer on brokers: {}, domains: {:?}, group_id: {}",
        brokers,
        domains,
        group_id
    );

    // connect to surrealdb
    let client = SurrealDB::new().await?;

    // Create the `FutureProducer` to produce kafka messages asynchronously.
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

    let routes = get_routes(client, producer);
    let socket = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", &socket.to_string());
    warp::serve(routes).run(socket).await;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "warp=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let matches = cli::get_matches();

    match matches.subcommand() {
        Some(("run", sub_matches)) => {
            run(sub_matches).await?;
        }
        _ => {
            unimplemented!();
        }
    }
    Ok(())
}
