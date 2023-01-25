use entrait::entrait;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::time::Duration;
use surrealdb::sql::{parse, Id, Query as SurrealQuery};
use surrealdb::sql::Value;
use surrealdb::{Datastore, Session};
use thiserror::Error;

#[cfg(test)]
mod tests {
    use crate::db::database::Query;

    use super::Database;

    #[tokio::test]
    async fn open_db_test() {
        let db = Database::new("memory", None, None).await;
        assert_eq!(db.is_ok(), true);
    }

    #[test]
    fn prepare_query_test() {
        let query = Database::prepare("select * from test");
        assert_eq!(query.is_ok(), true);
    }

    #[tokio::test]
    async fn execute_raw_test() {
        let db = Database::new("memory", None, None).await.unwrap();
        let query: Query = "select * from test".into();
        let _res = db.execute(query, None).await.unwrap();

        dbg!(_res);
    }

    #[tokio::test]
    async fn execute_query_test() {
        let query = Database::prepare("select * from test").unwrap();
        let db = Database::new("memory", None, None).await.unwrap();
        let _res = db.execute(query, None).await.unwrap();
        dbg!(_res);
    }
}

#[entrait(pub GetDb)]
pub fn get_db(db: &Database) -> &Database {
    db
}

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Database open error")]
    OpenError(#[source] surrealdb::Error),
    #[error("Database query error")]
    QueryError(#[source] surrealdb::Error),
    #[error("Database query prepare error")]
    QueryPrepareError(#[source] surrealdb::Error),
    #[error("Database query internal error")]
    InternalQueryError(#[source] surrealdb::Error),
}

pub type Variables = Option<BTreeMap<String, Value>>;
pub type QueryResult = Result<Vec<Response>, DatabaseError>;
pub type PrepareResult = Result<Query, DatabaseError>;

pub struct Database {
    ds: Datastore,
    ses: Session,
}

impl Database {
    pub async fn new(
        path: &str,
        ns: Option<&str>,
        db: Option<&str>,
    ) -> Result<Self, DatabaseError> {
        let mut ses = Session::for_kv()
            .with_ns(ns.unwrap_or("root"))
            .with_db(db.unwrap_or("root"));
        ses.rt = true;
        return Datastore::new(path)
            .await
            .map(|ds| Database { ds, ses })
            .map_err(|err| DatabaseError::OpenError(err));
    }

    pub fn prepare(query: &str) -> PrepareResult {
        parse(query)
            .map_err(|err| DatabaseError::QueryPrepareError(err))
            .map(|query| Query::Prepared(query))
    }

    pub async fn execute(&self, query: Query, vars: Variables) -> QueryResult {
        let fut = match query {
            Query::Raw(str) => self.ds.execute(str.as_str(), &self.ses, vars, false).await,
            Query::Prepared(query) => self.ds.process(query, &self.ses, vars, false).await,
        };
        let res = fut
            .map_err(|err| DatabaseError::QueryError(err))
            .map(|val| {
                val.into_iter()
                    .map(|v| Response::try_from(v))
                    .collect::<Result<Vec<Response>, DatabaseError>>()
            });
        match res {
            Err(err) => Err(err),
            Ok(val) => match val {
                Err(err) => Err(err),
                Ok(val) => Ok(val),
            },
        }
    }

    pub fn next_id() -> Id {
        Id::rand()
    }
}

#[derive(Debug)]
pub enum Query {
    Raw(String),
    Prepared(SurrealQuery),
}

impl From<&str> for Query {
    fn from<'a>(value: &'a str) -> Self {
        Query::Raw(value.into())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub sql: Option<String>,
    pub time: Duration,
    pub result: Value,
}

impl TryFrom<surrealdb::Response> for Response {
    type Error = DatabaseError;

    fn try_from(value: surrealdb::Response) -> Result<Self, Self::Error> {
        let surrealdb::Response { sql, time, result } = value;
        match result {
            Err(err) => Err(DatabaseError::InternalQueryError(err)),
            Ok(v) => Ok(Response {
                sql,
                time,
                result: v,
            }),
        }
    }
}
