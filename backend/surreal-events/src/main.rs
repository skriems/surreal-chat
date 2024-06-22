// #![deny(warnings)]
mod chat;
mod cli;
mod events;
mod process;
mod processor;

use anyhow::Result;
use clap::ArgMatches;
use futures::stream::FuturesUnordered;
use futures_util::StreamExt;
use processor::processor;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use lib::client::SurrealDB;

async fn run(matches: &ArgMatches) -> Result<()> {
    // connect to surrealdb
    let db = SurrealDB::new().await?;

    let brokers = matches
        .get_one::<String>("brokers")
        .expect("brokers argument is required");
    let group_id = matches
        .get_one::<String>("group-id")
        .expect("group_id argument is required");
    let input_topic = matches
        .get_one::<String>("input-topic")
        .expect("input-topic argument is required");
    let output_topic = matches
        .get_one::<String>("output-topic")
        .expect("output-topic argument is required");
    let num_workers = matches
        .get_one::<usize>("workers")
        .expect("workers argument is required");

    (0..*num_workers)
        .map(|_| {
            tokio::spawn(processor(
                brokers.to_owned(),
                group_id.to_owned(),
                input_topic.to_owned(),
                output_topic.to_owned(),
                db.clone(),
            ))
        })
        .collect::<FuturesUnordered<_>>()
        .for_each(|_| async { () })
        .await;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "surreal_events=trace".into()),
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
