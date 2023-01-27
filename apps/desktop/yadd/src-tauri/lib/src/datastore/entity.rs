use serde::{Serialize, de::DeserializeOwned};
use surrealdb::sql::{Value, thing, Id};

use super::{Database, database::Query, utils::Parse};
use super::macros::vars;

pub type EntityRef = surrealdb::sql::Thing;

pub trait EntityRefExt {
    fn try_parse(value: &str) -> anyhow::Result<EntityRef>  {
        Ok(thing(value)?)
    }

    fn from_entity<T>(entity: &T) -> Option<EntityRef> where T: Entity {
        entity.id().and_then(|id| EntityRef::try_parse(id.as_str()).ok())
    }

    fn with_id<T>(id: &str) -> EntityRef where T: Entity {
        let collection = T::collection();
        EntityRef::from((collection.as_str(), id))
    }

    fn with_rand_id<T>() -> EntityRef where T: Entity {
        let collection = T::collection();
        EntityRef::from((collection, Id::rand().to_raw()))
    }

    fn table(&self) -> String;
    fn id(&self) -> String;
}

impl EntityRefExt for EntityRef {
    fn table(&self) -> String {
        self.tb.clone()
    }

    fn id(&self) -> String {
        self.id.to_raw()
    }
}

pub trait Entity: Serialize + DeserializeOwned {
    fn id(&self) -> Option<String>;

    fn collection() -> String;

    fn extract(value: surrealdb::sql::Value) -> anyhow::Result<Self> {
        let temp_value = serde_json::to_value(value)?;
        let ent = serde_json::from_value::<Self>(temp_value)?;
        Ok(ent)
    }
}

#[async_trait::async_trait(?Send)]
pub trait PersistentEntity {
    type Ent;

    async fn select(reference: EntityRef,db: &Database) -> anyhow::Result<Self::Ent>;
    async fn create(&self,db: &Database) -> anyhow::Result<Self::Ent>;
    async fn upsert(&self,db: &Database) -> anyhow::Result<Self::Ent>;
    async fn update(&self, db: &Database) -> anyhow::Result<Self::Ent>;
    async fn delete(&self, db: &Database) -> anyhow::Result<()>;
}

#[async_trait::async_trait(?Send)]
impl<T> PersistentEntity for T where T: Entity {
    type Ent = T;

    async fn select(reference: EntityRef, db: &Database) -> anyhow::Result<Self::Ent> {
        let sql: Query = Query::from("SELECT * FROM type::thing($table, $id)");
        let vars = vars! {
            String::from("table") => Value::from(reference.table()),
            String::from("id") => Value::from(reference.id()),
        };
        let mut res = db.execute(sql, Some(vars)).await?;
        let res =  res.remove(0);
        let mut res = serde_json::from_value::<Vec<T>>(serde_json::to_value(res.result)?)?;
        Ok(res.remove(0))
    }

    async fn create(&self, db: &Database) -> anyhow::Result<Self::Ent> {
        let id = self.id();
        if id.is_some() {
            return Err(anyhow::anyhow!("entity has id"));
        }
        let sql: Query = Query::from("CREATE type::thing($table, $id) CONTENT $data;");
        let vars = vars! {
            String::from("table") => Value::from(T::collection()),
            String::from("id") => Value::from(self.id().unwrap_or(Database::next_id().to_raw())),
            String::from("data") => Value::parse(serde_json::to_string(self).unwrap().as_str()),
        };
        let mut res = db.execute(sql, Some(vars)).await?;
        let res =  res.remove(0);
        let mut res = serde_json::from_value::<Vec<T>>(serde_json::to_value(res.result)?)?;
        Ok(res.remove(0))
    }

    async fn upsert(&self,db: &Database) -> anyhow::Result<Self::Ent> {
        let table = T::collection();
        let sql = Query::from(format!("INSERT INTO {} $data;", table).as_str());
        let vars = vars! {
            String::from("data") => Value::parse(serde_json::to_string(self).unwrap().as_str()),
        };
        let mut res = db.execute(sql, Some(vars)).await?;
        let res =  res.remove(0);
        let mut res = serde_json::from_value::<Vec<T>>(serde_json::to_value(res.result)?)?;
        Ok(res.remove(0))
    }

    async fn update(&self, db: &Database) -> anyhow::Result<Self::Ent> {
        if self.id().is_none() {
            return Err(anyhow::anyhow!("empty id"));
        }
        let reference = EntityRef::from_entity(self).unwrap();
        let sql = Query::from("UPDATE type::thing($table, $id) CONTENT $data");
        let vars = vars! {
            String::from("table") => Value::from(reference.table()),
            String::from("id") => Value::from(reference.id()),
            String::from("data") => Value::parse(serde_json::to_string(self).unwrap().as_str()),
        };
        let mut res = db.execute(sql, Some(vars)).await?;
        let res =  res.remove(0);
        let mut res = serde_json::from_value::<Vec<T>>(serde_json::to_value(res.result)?)?;
        Ok(res.remove(0))
    }

    async fn delete(&self, db: &Database) -> anyhow::Result<()> {
        if self.id().is_none() {
            return Ok(());
        }
        let sql = Query::from("DELETE type::thing($table, $id)");
        let vars = vars! {
            String::from("table") => Value::from(T::collection()),
            String::from("id") => Value::from(self.id().unwrap()),
        };
        db.execute(sql, Some(vars)).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use serde::{Serialize, Deserialize};

    use super::{Entity, Database, PersistentEntity, Query, EntityRef, EntityRefExt};

    

    #[derive(Debug, Default, Serialize, Deserialize)]
    struct Someth {
        #[serde(skip_serializing_if = "Option::is_none")]
        id: Option<String>,
        count: i32,
    }

    impl Entity for Someth {
        fn id(&self) -> Option<String> {
            self.id.clone()
        }

        fn collection() -> String {
            String::from("someth")
        }
    }

    #[test]
    fn test_entity_ref_parse() {
        let ent_ref = EntityRef::try_parse("table:1").unwrap();
        dbg!(&ent_ref);
    }

    #[test]
    fn test_entity_ref_from_entity() {
        let ent = Someth {id: Some("someth:1".to_string()), count: 0};
        assert_eq!(EntityRef::from_entity(&ent).is_some(), true);

        let ent = Someth {id: None, count: 0};
        assert_eq!(EntityRef::from_entity(&ent).is_some(), false);
    }

    #[test]
    fn test_entity_ref_with_id() {
        assert_eq!(EntityRef::with_id::<Someth>("1").to_raw(), "someth:1");
    }

    #[test]
    fn test_entity_ref_with_rand_id() {
        let ent_ref = EntityRef::with_rand_id::<Someth>();
        dbg!(&ent_ref);
    }

    #[async_std::test]
    async fn test_full() {
        let db = Database::new("memory", None, None).await.unwrap();
        let ent = Someth {id: None, count: 0};
        let mut ent = ent.create(&db).await.unwrap();
        ent.count = ent.count + 1;
        let mut ent = ent.upsert(&db).await.unwrap();
        ent.count = ent.count + 1;
        let ent = ent.update(&db).await.unwrap();
        let ent_ref = EntityRef::from_entity(&ent).unwrap();
        let selected_ent = Someth::select(ent_ref, &db).await.unwrap();
        assert_eq!(ent.count, 1);
        assert_eq!(selected_ent.count, 1);
        ent.delete(&db).await.unwrap();
    }

    #[async_std::test]
    async fn test_full_2() {
        let db = Database::new("memory", None, None).await.unwrap();
        let ent = Someth {id: None, count: 0};
        let mut ent = ent.upsert(&db).await.unwrap();
        ent.count = ent.count + 1;
        let mut ent = ent.update(&db).await.unwrap();
        ent.count = ent.count + 1;
        let ent = ent.update(&db).await.unwrap();
        let ent_ref = EntityRef::from_entity(&ent).unwrap();
        let selected_ent = Someth::select(ent_ref, &db).await.unwrap();
        assert_eq!(ent.count, 2);
        assert_eq!(selected_ent.count, 2);
        ent.delete(&db).await.unwrap();
    }

    #[async_std::test]
    async fn test_select() {
        let db = Database::new("memory", None, None).await.unwrap();
        let ent_ref = EntityRef::with_rand_id::<Someth>();
        let ent = Someth{id: Some(ent_ref.to_raw()), count: 0};
        ent.upsert(&db).await.unwrap();

        let res = Someth::select(ent_ref, &db).await.unwrap();
        dbg!(&res);
    }

    #[async_std::test]
    async fn test_create() {
        let db = Database::new("memory", None, None).await.unwrap();
        let ent = Someth{id: Some("1".into()), count: 0};
        let res = ent.create(&db).await;
        assert_eq!(res.is_err(), true);

        let ent = Someth{id: None, count: 0};
        ent.create(&db).await.unwrap();
        let res = db.execute(Query::from("select * from someth;"), None).await.unwrap();
        dbg!(&res);
    }

    #[async_std::test]
    async fn test_upsert() {
        let db = Database::new("memory", None, None).await.unwrap();
        let ent = Someth{id: Some("1".into()), count: 0};
        ent.upsert(&db).await.unwrap();

        let ent = Someth{id: None, count: 0};
        ent.upsert(&db).await.unwrap();
        let res = db.execute(Query::from("select * from someth;"), None).await.unwrap();
        dbg!(&res);
    }

    #[async_std::test]
    async fn test_update() {
        let db = Database::new("memory", None, None).await.unwrap();
        let ent_ref = EntityRef::with_id::<Someth>("1");
        let ent = Someth{id: Some(ent_ref.to_raw()), count: 0};
        ent.upsert(&db).await.unwrap();

        let ent = Someth{id: Some(ent_ref.to_raw()), count: 1};
        ent.update(&db).await.unwrap();
        let res = db.execute(Query::from("select * from someth;"), None).await.unwrap();
        dbg!(&res);
    }

    #[async_std::test]
    async fn test_delete() {
        let db = Database::new("memory", None, None).await.unwrap();
        let ent_ref = EntityRef::with_id::<Someth>("1");
        let ent = Someth{id: Some(ent_ref.to_raw()), count: 0};
        ent.upsert(&db).await.unwrap();

        let ent = Someth{id: Some(ent_ref.to_raw()), count: 1};
        ent.delete(&db).await.unwrap();
        let res = db.execute(Query::from("select * from someth;"), None).await.unwrap();
        dbg!(&res);
    }
}