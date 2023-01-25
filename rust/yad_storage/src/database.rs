use entrait::entrait_export;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;
use surrealdb::sql::Value;
use surrealdb::sql::{parse, Id, Query as SurrealQuery};
use surrealdb::{Datastore, Session};
use thiserror::Error;

use crate::repository::Repository;

#[cfg(test)]
mod tests {
    use crate::database::Query;

    use super::Database;

    #[async_std::test]
    async fn open_db_test() {
        let db = Database::new("memory", None, None).await;
        assert_eq!(db.is_ok(), true);
    }

    #[test]
    fn prepare_query_test() {
        let query = Database::prepare("select * from test");
        assert_eq!(query.is_ok(), true);
    }

    #[async_std::test]
    async fn execute_raw_test() {
        let db = Database::new("memory", None, None).await.unwrap();
        let query: Query = "select * from test".into();
        let _res = db.execute(query, None).await.unwrap();

        dbg!(_res);
    }

    #[async_std::test]
    async fn execute_query_test() {
        let query = Database::prepare("select * from test").unwrap();
        let db = Database::new("memory", None, None).await.unwrap();
        let _res = db.execute(query, None).await.unwrap();
        dbg!(_res);
    }
}

#[entrait_export(pub GetDb)]
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
    #[error("Database transaction create error")]
    CreateTransactionError(#[source] surrealdb::Error),
    #[error("Database transaction commit failed")]
    CommitError(#[source] surrealdb::Error),
    #[error("Database transaction rollback failed")]
    RollbackError(#[source] surrealdb::Error),
    #[error("Database repository creation failed {0}")]
    RepositoryCreateFailed(String),

}

pub type Variables = Option<BTreeMap<String, Value>>;
pub type QueryResult = Result<Vec<Response>, DatabaseError>;
pub type PrepareResult = Result<Query, DatabaseError>;

#[derive(Clone)]
pub struct Database {
    ds: Arc<Datastore>,
    ses: Session,
}

impl std::fmt::Debug for Database {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Database").field("ses", &self.ses).finish()
    }
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
            .map(|ds| Database { ds: Arc::new(ds), ses })
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

    pub fn repository(&self) -> Repository {
        Repository::new(self.clone())
    }
}

#[derive(Debug)]
pub enum Query {
    Raw(String),
    Prepared(SurrealQuery),
}

impl From<&str> for Query {
    fn from(value: &str) -> Self {
        Query::Raw(value.into())
    }
}

pub struct Transaction(surrealdb::Transaction);

impl Transaction {
    pub async fn commit(&mut self) -> Result<(), DatabaseError> {
        self.0.commit().await.map_err(|err| DatabaseError::CommitError(err))
    }

    pub async fn rollback(&mut self) -> Result<(), DatabaseError>  {
        self.0.cancel().await.map_err(|err| DatabaseError::RollbackError(err))
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
