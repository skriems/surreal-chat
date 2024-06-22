use std::time::Duration;

use anyhow::Result;
use futures_util::TryStreamExt;
use rdkafka::{
    consumer::{Consumer, StreamConsumer},
    message::OwnedMessage,
    producer::{FutureProducer, FutureRecord},
    ClientConfig, Message,
};
use tracing;

use lib::{client::SurrealDB, surreal::EventMessage};

use crate::process::process;

// async fn record_borrowed_message_receipt(msg: &BorrowedMessage<'_>) {
//     // Simulate some work that must be done in the same order as messages are
//     // received; i.e., before truly parallel processing can begin.
//     tracing::info!("Message received: {}", msg.offset());
// }
//
// async fn record_owned_message_receipt(_msg: &OwnedMessage) {
//     // Like `record_borrowed_message_receipt`, but takes an `OwnedMessage`
//     // instead, as in a real-world use case  an `OwnedMessage` might be more
//     // convenient than a `BorrowedMessage`.
// }

async fn process_message<'a>(msg: OwnedMessage, db: SurrealDB) -> Result<Option<EventMessage>> {
    tracing::info!("Expensive computation running on message {}", msg.offset());

    match msg.payload_view::<str>() {
        Some(Ok(payload)) => {
            let event = serde_json::from_str::<EventMessage>(payload)?;
            process(event, "TODO: implement another trait", db).await
        }
        Some(Err(e)) => {
            tracing::error!("Message payload is not a string: {:?}", e);
            Ok(None)
        }
        None => {
            tracing::warn!("No payload");
            Ok(None)
        }
    }
}

pub async fn processor(
    brokers: String,
    group_id: String,
    input_topic: String,
    output_topic: String,
    db: SurrealDB,
) {
    tracing::info!(
        "Starting kafka worker on brokers: {}, group_id: {}, input_topic: {}, output_topic: {}",
        brokers,
        group_id,
        input_topic,
        output_topic
    );

    // Create the `StreamConsumer`, to receive the messages from the topic in form of a `Stream`.
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", &group_id)
        .set("bootstrap.servers", &brokers)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "false")
        .create()
        .expect("Consumer creation failed");

    consumer
        .subscribe(&[&input_topic])
        .expect("Can't subscribe to specified topic");

    // Create the `FutureProducer` to produce asynchronously.
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", &brokers)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

    // Create the outer pipeline on the message stream.
    let stream_processor = consumer.stream().try_for_each(|borrowed_message| {
        let producer = producer.clone();
        let output_topic = output_topic.to_string();
        let db = db.clone();

        async move {
            // Process each message
            // record_borrowed_message_receipt(&borrowed_message).await;
            // Borrowed messages can't outlive the consumer they are received from, so they need to
            // be owned in order to be sent to a separate thread.
            let owned_message = borrowed_message.detach();
            // record_owned_message_receipt(&owned_message).await;

            tokio::spawn(async move {
                // The body of this block will be executed on the main thread pool,
                // but we perform `expensive_computation` on a separate thread pool
                // for CPU-intensive tasks via `tokio::task::spawn_blocking`.
                // let event = tokio::task::spawn_blocking(|| process_message(owned_message, db))
                let event = tokio::task::spawn(async { process_message(owned_message, db).await })
                    .await
                    .expect("failed to wait for process_message");

                match event {
                    Ok(Some(event)) => match serde_json::to_string(&event) {
                        Ok(json) => {
                            tracing::debug!("handling event: {}", json);
                            match producer
                                .send(
                                    FutureRecord::to(&output_topic).key("events").payload(&json),
                                    Duration::from_secs(0),
                                )
                                .await
                            {
                                Ok(delivery) => tracing::debug!("kafka message sent: {:?}", delivery),
                                Err((e, _)) => tracing::error!("Error: {:?}", e),
                            }
                        }
                        Err(e) => {
                            tracing::error!("Error serializing event: {:?}", e);
                        }
                    },
                    Ok(None) => {
                        tracing::debug!("No event produced");
                    }
                    Err(e) => {
                        tracing::error!("Error processing message: {:?}", e);
                    }
                }
            });
            Ok(())
        }
    });

    tracing::info!("Starting event loop");
    stream_processor.await.expect("stream processing failed");
    tracing::info!("Stream processing terminated");
}
