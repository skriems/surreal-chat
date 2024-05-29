use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DBUser {
    pub id: Thing,
    pub username: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DBUserChanged {
    pub user: Thing,
    pub data: DBUser,
}
