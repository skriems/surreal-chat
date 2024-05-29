use db::DBUser;
use messages::CreateUser;
use surrealdb::Result;

use crate::client::SurrealDB;

pub mod db;
pub mod messages;

pub async fn create_user(db: &SurrealDB, username: &str) -> Result<Option<DBUser>> {
    let users: Vec<DBUser> = db
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

pub async fn get_user(db: &SurrealDB, username: &str) -> Result<Option<DBUser>> {
    let sql = "SELECT * FROM type::table($table) WHERE username = $username";
    let mut res = db
        .client
        .query(sql)
        .bind(("table", "user"))
        .bind(("username", username))
        .await?;
    let users: Vec<DBUser> = res.take(0)?;
    tracing::debug!("get_user.users {:?}", &users);
    if let Some(user) = users.first() {
        Ok(Some(user.to_owned()))
    } else {
        Ok(None)
    }
}

pub async fn get_users(db: &SurrealDB) -> Result<Vec<DBUser>> {
    db.client.select("user").await
}
