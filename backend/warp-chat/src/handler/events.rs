use anyhow::Result;
use lib::client::SurrealDB;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use surrealdb::sql::Thing;

// The query parameters for list_todos.
// #[derive(Debug, Deserialize)]
// pub struct ListOptions {
//     pub offset: Option<usize>,
//     pub limit: Option<usize>,
// }

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Foo {
    id: Thing,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Fara {
    #[serde(with = "lib::serde::string_thing")]
    id: Thing,
}

impl From<Foo> for Fara {
    fn from(foo: Foo) -> Self {
        Self { id: foo.id }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
enum Event {
    Foo(Foo),
    Fara(Foo),
}

pub async fn get_events(db: SurrealDB) -> Result<impl warp::Reply, Infallible> {
    let result: surrealdb::Result<Vec<Foo>> = db.client.select("event").await;
    tracing::debug!("get_events.result: {:?}", &result);

    if let Ok(events) = result {
        let events: Vec<Fara> = events.into_iter().map(|foo| foo.into()).collect();
        return Ok(warp::reply::json(&events));
    }
    let v: Vec<Fara> = vec![];
    Ok(warp::reply::json(&v))
}
