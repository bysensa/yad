use async_trait::async_trait;
use serde::{Serialize, de::DeserializeOwned};
use surrealdb::sql::Value;

use crate::{Database, database::Query, utils::Parse};


pub trait Entity: Serialize + DeserializeOwned {
    fn id(&self) -> Option<String>;

    fn collection() -> String;

    fn reference_from(id: String) -> EntityRef {
        EntityRef(Self::collection(), id)
    }

    fn reference(&self) -> Option<EntityRef> {
        self.id().map(|v| EntityRef(Self::collection(), v))
    }
}

pub struct EntityRef(String,String);

impl EntityRef {
    pub fn id(&self) -> &str {
        self.1.as_str()
    }

    pub fn collection(&self) -> &str {
        self.0.as_str()
    }
}

#[async_trait::async_trait(?Send)]
pub trait PersistentEntity {
    async fn create(&self,db: &Database) -> anyhow::Result<()>;
    async fn upsert(&self,db: &Database) -> anyhow::Result<()>;
    async fn update(&self, db: &Database) -> anyhow::Result<()>;
    async fn delete(&self, db: &Database) -> anyhow::Result<()>;
}

#[async_trait::async_trait(?Send)]
impl<T> PersistentEntity for T where T: Entity {
    async fn create(&self, db: &Database) -> anyhow::Result<()> {
        let id = self.id();
        if id.is_some() {
            return Ok(());
        }
        let sql: Query = Query::from("CREATE type::thing($table, $id) CONTENT $data;");
        let vars = crate::vars! {
            String::from("table") => Value::from(T::collection()),
            String::from("id") => Value::from(self.id().unwrap_or(Database::next_id().to_raw())),
            String::from("data") => Value::parse(serde_json::to_string(self).unwrap().as_str()),
        };
        db.execute(sql, Some(vars)).await?;
        Ok(())
    }

    async fn upsert(&self,db: &Database) -> anyhow::Result<()> {
        let table = T::collection();
        let sql = Query::from(format!("INSERT INTO {} $data;", table).as_str());
        let vars = crate::vars! {
            String::from("data") => Value::parse(serde_json::to_string(self).unwrap().as_str()),
        };
        db.execute(sql, Some(vars)).await?;
        Ok(())
    }

    async fn update(&self, db: &Database) -> anyhow::Result<()> {
        if self.id().is_none() {
            return Ok(());
        }
        let sql = Query::from("UPDATE type::thing($table, $id) CONTENT $data");
        let vars = crate::vars! {
            String::from("table") => Value::from(T::collection()),
            String::from("id") => Value::from(self.id().unwrap()),
            String::from("data") => Value::parse(serde_json::to_string(self).unwrap().as_str()),
        };
        db.execute(sql, Some(vars)).await?;
        Ok(())
    }

    async fn delete(&self, db: &Database) -> anyhow::Result<()> {
        if self.id().is_none() {
            return Ok(());
        }
        let sql = Query::from("DELETE type::thing($table, $id)");
        let vars = crate::vars! {
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

    use crate::{Entity, Database, PersistentEntity, database::Query};

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

    #[async_std::test]
    async fn test_create() {
        let db = Database::new("memory", None, None).await.unwrap();
        let ent = Someth{id: Some("1".into()), count: 0};
        ent.create(&db).await.unwrap();

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
        let ent = Someth{id: Some("1".into()), count: 0};
        ent.create(&db).await.unwrap();

        let ent = Someth{id: Some("1".into()), count: 1};
        ent.update(&db).await.unwrap();
        let res = db.execute(Query::from("select * from someth;"), None).await.unwrap();
        dbg!(&res);
    }

    #[async_std::test]
    async fn test_delete() {
        let db = Database::new("memory", None, None).await.unwrap();
        let ent = Someth{id: Some("1".into()), count: 0};
        ent.create(&db).await.unwrap();

        let ent = Someth{id: Some("1".into()), count: 1};
        ent.delete(&db).await.unwrap();
        let res = db.execute(Query::from("select * from someth;"), None).await.unwrap();
        dbg!(&res);
    }
}