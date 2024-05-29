use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use surrealdb::Result;

use lib::events::Event;
use lib::{client::SurrealDB, users::User};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Thing,
    user: Thing,
    events: Vec<Event>,
    finished: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransactionInput {
    user: Thing,
    events: Vec<Event>,
    finished: bool,
}

pub async fn create_transaction(db: &SurrealDB, user: &User) -> Result<Option<Transaction>> {
    let transactions: Vec<Transaction> = db
        .client
        .create("transaction")
        .content(TransactionInput {
            // user: format!("user:{}", user.id.id.to_string()),
            user: user.id.clone(),
            events: Vec::new(),
            finished: false,
        })
        .await?;

    if let Some(transaction) = transactions.first() {
        Ok(Some(transaction.to_owned()))
    } else {
        Ok(None)
    }
}

pub async fn get_transaction(db: &SurrealDB, id: &str) -> Result<Option<Transaction>> {
    db.client.select(("transaction", id)).await
}

pub async fn update_transaction(
    db: &SurrealDB,
    id: &str,
    event: Event,
) -> Result<Option<Transaction>> {
    let mut transaction = get_transaction(db, id).await?;
    if let Some(transaction) = transaction.as_mut() {
        transaction.events.push(event);
        db.client
            .update(("transaction", id))
            .content(transaction)
            .await
    } else {
        Ok(None)
    }
}
