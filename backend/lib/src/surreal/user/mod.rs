pub mod models;

use models::DBUserMessageData;
use serde::{Deserialize, Serialize};
use surrealdb::Result;

use crate::client::SurrealDB;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateUser {
    pub username: String,
}

pub async fn create_user(db: &SurrealDB, username: &str) -> Result<Option<DBUserMessageData>> {
    let users: Vec<DBUserMessageData> = db
        .client
        .create("user")
        .content(CreateUser {
            username: username.to_owned(),
        })
        .await?;

    tracing::debug!("create_user.users {:?}", &users);
    if let Some(user) = users.first() {
        Ok(Some(user.to_owned()))
    } else {
        Ok(None)
    }
}

pub async fn get_user(db: &SurrealDB, username: &str) -> Result<Option<DBUserMessageData>> {
    let sql = "SELECT * FROM type::table($table) WHERE username = $username";
    let mut res = db
        .client
        .query(sql)
        .bind(("table", "user"))
        .bind(("username", username))
        .await?;
    let users: Vec<DBUserMessageData> = res.take(0)?;
    tracing::debug!("get_user.users {:?}", &users);
    if let Some(user) = users.first() {
        Ok(Some(user.to_owned()))
    } else {
        Ok(None)
    }
}

pub async fn get_users(db: &SurrealDB) -> Result<Vec<DBUserMessageData>> {
    db.client.select("user").await
}

pub async fn get_or_create_user(
    db: &SurrealDB,
    username: &str,
) -> Result<Option<DBUserMessageData>> {
    if let Some(user) = get_user(db, username).await? {
        Ok(Some(user))
    } else {
        create_user(db, username).await
    }
}
