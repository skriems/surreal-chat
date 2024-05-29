use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::serde::string_thing;

use super::db::{DBUser, DBUserChanged};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(with = "string_thing")]
    pub id: Thing,
    pub username: String,
}

impl From<DBUser> for User {
    fn from(db: DBUser) -> Self {
        Self {
            id: db.id,
            username: db.username,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub username: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserChanged {
    #[serde(with = "string_thing")]
    pub user: Thing,
    pub data: User,
}

impl From<DBUserChanged> for UserChanged {
    fn from(db: DBUserChanged) -> Self {
        Self {
            user: db.user,
            data: db.data.into(),
        }
    }
}
