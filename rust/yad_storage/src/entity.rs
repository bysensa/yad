use std::collections::BTreeMap;

use entrait::entrait;
use serde_json::{from_value, to_value};
use surrealdb::sql::Value;

use super::{database::Query, persistent::Persistent, utils::Parse, GetDb};

#[entrait(pub GetEntity)]
pub async fn get_entity(deps: &impl GetDb, id: String) -> anyhow::Result<Persistent> {
    let db = deps.get_db();
    let query: Query = "select * from $identity;;".into();

    let identity = Value::parse(id.as_str());

    let identity_entry = ("identity".to_string(), identity);
    let vars = BTreeMap::from([identity_entry]);

    let mut res = db.execute(query, Some(vars)).await.unwrap();
    let res = res.remove(0);
    let res = res.result;
    let mut res = from_value::<Vec<serde_json::Value>>(to_value(res)?)?;
    let res = res.remove(0);
    Ok(Persistent::from_raw(id, res))
}

#[entrait(pub CreateEntity)]
pub async fn create_entity(deps: &impl GetDb, entity: Persistent) -> anyhow::Result<Persistent> {
    let db = deps.get_db();
    let query: Query = "create $identity content $content;".into();

    let identity = Value::parse(entity.id().as_str());
    let content = Value::parse(entity.to_string()?.as_str());

    let identity_entry = ("identity".to_string(), identity);
    let content_entry = ("content".to_string(), content);
    let vars = BTreeMap::from([identity_entry, content_entry]);

    let mut res = db.execute(query, Some(vars)).await.unwrap();
    let res = res.remove(0);
    let res = res.result;
    let mut res = from_value::<Vec<serde_json::Value>>(to_value(res)?)?;
    let res = res.remove(0);
    Ok(entity.with_value(res))
}

#[entrait(pub UpdateEntity)]
pub async fn update_entity(deps: &impl GetDb, entity: Persistent) -> anyhow::Result<Persistent> {
    let db = deps.get_db();
    let query: Query = "update $identity content $content;".into();

    let identity = Value::parse(entity.id().as_str());
    let content = Value::parse(entity.to_string()?.as_str());

    let identity_entry = ("identity".to_string(), identity);
    let content_entry = ("content".to_string(), content);
    let vars = BTreeMap::from([identity_entry, content_entry]);

    let mut res = db.execute(query, Some(vars)).await.unwrap();
    let res = res.remove(0);
    let res = res.result;
    let mut res = from_value::<Vec<serde_json::Value>>(to_value(res)?)?;
    let res = res.remove(0);
    Ok(entity.with_value(res))
}
