use async_trait::async_trait;
use serde::{Serialize, de::DeserializeOwned};
use surrealdb::sql::Value;


use crate::{Database, database::{Transaction, Query}, utils::Parse};

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

#[async_trait]
pub trait PersistentEntity {

}



pub struct Repository {
    db: Database,
}

impl Repository {
    pub fn new(db: Database) -> Self {
        Repository {db}
    }

    pub async fn select(&self, reference: EntityRef) {

    }

    pub async fn create<E>(&self, entity: &E) where E: Entity {
        let data = serde_json::to_string(&entity).unwrap();
        let EntityRef(table, id) = entity.reference().unwrap_or(E::reference_from(Database::next_id().to_string()));

        let sql: Query = Query::from("CREATE type::thing($table, $id) CONTENT $data");
        let vars = crate::vars! {
            String::from("table") => Value::from(table),
            String::from("id") => Value::from(id),
            String::from("data") => Value::parse(data.as_str()),
        };
        self.db.execute(sql, Some(vars)).await.unwrap();
    }

    pub async fn upsert<E>(&self, entity: E) where E: Entity {}

    pub async fn update<E>(&self, entity: E) where E: Entity {}

    pub async fn modify(&self, reference: EntityRef) {}

    pub async fn delete(&self, reference: EntityRef) {}

    pub async fn commit(self) {}

    pub async fn rollback(self) {}
}

#[cfg(test)]
mod tests {
    use serde::{Serialize, Deserialize};

    use crate::Database;

    use super::{Entity};

    #[derive(Debug, Serialize, Deserialize)]
    struct Someth {}

    impl Entity for Someth {
        fn id(&self) -> Option<String> {
            Some(String::from("id"))
        }

        fn collection() -> String {
            String::from("someth")
        }
    }

    #[async_std::test]
    async fn test_create() {
        let db = Database::new("memory", None, None).await.unwrap();
        let repo = db.repository();
        let ent = Someth{};
        repo.create(&ent).await;
        repo.commit().await;
    }

    #[async_std::test]
    async fn test_upsert() {
        let db = Database::new("memory", None, None).await.unwrap();
        let repo = db.repository();
        let ent = Someth{};
        repo.upsert(ent).await;
    }

    #[async_std::test]
    async fn test_update() {
        let db = Database::new("memory", None, None).await.unwrap();
        let repo = db.repository();
        let ent = Someth{};
        repo.update(ent).await;
    }
}