use derive_more::{Display, From};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::str::FromStr;
use sqlx::Postgres;

#[derive(Debug, thiserror::Error)]
pub enum DataError {
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
}

/// Define general-use types for database access.
pub type AppDatabase = Database<Postgres>;
pub type DatabasePool = sqlx::postgres::PgPool;
pub type Transaction<'t> = sqlx::Transaction<'t, Postgres>;
pub type AppDatabaseRow = sqlx::postgres::PgRow;
pub type AppQueryResult = sqlx::postgres::PgQueryResult;

pub struct Database<D: sqlx::Database>(sqlx::Pool<D>);
pub mod model;
pub mod query;

impl Database<Postgres> {
    pub async fn new(connection_str: &str) -> Self {
        /// Establishes a connection with the database.
        let pool = sqlx::postgres::PgPoolOptions::new()
            .connect(connection_str)
            .await;

        match pool {
            Ok(pool) => Self(pool),
            Err(e) => {
                eprintln!("{:?}\n", e);
                eprintln!("If the database has not been created,\
                           run \n $ sqlx database setup \n");
            panic!("database connection error")},
        }
    }

    pub fn get_pool(&self) -> &DatabasePool {
        &self.0
    }
}

#[derive(Clone, Debug, From, Display, Deserialize, Serialize)]
/// Struct to generate the unique identifiers of the clips.
pub struct DbId(Uuid);

impl DbId {
    pub fn new() -> Self {
        Uuid::new_v4().into()
    }

    pub fn nil() -> Self {
        Self(Uuid::nil())
    }
}

impl Default for DbId {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for DbId {
    type Err = uuid::Error;
    fn from_str(id: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(id)?))
    }
}

impl From<DbId> for String {
    fn from(id: DbId) -> Self {
        format!("{}", id.0)
    }
}

#[cfg(test)]
pub mod test {
    use crate::data::*;
    use tokio::runtime::Handle;

    pub fn new_db(handle: &Handle) -> AppDatabase {
        use sqlx::migrate::Migrator;
        use std::path::Path;

        handle.block_on(async move {
            let db = Database::new(":memory:").await;
            let migrator = Migrator::new(Path::new("./migrations")).await.unwrap();


            let pool = db.get_pool();
            migrator.run(pool).await.unwrap();

            db
        })
    }
}