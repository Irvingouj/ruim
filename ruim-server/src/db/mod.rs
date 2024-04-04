
mod user;
pub mod chat;
use axum::extract::FromRef;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Postgres};
use sqlx::{Connection, Pool};


use crate::context::RuimContext;

#[derive(Clone)]
pub struct Database {
    pool: Pool<Postgres>
}

impl Database {
    pub async fn new() -> anyhow::Result<Self> {
        let database_url = std::env::var("DATABASE_URL")?;
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;

        Ok(Self { pool })
    }
}

impl FromRef<RuimContext> for Database{
    fn from_ref(input: &RuimContext) -> Self {
        input.db.clone()
    }
}


#[derive(Debug, thiserror::Error)]
pub enum DBError {
    #[error("sqlx error: {0}")]
    Sqlx(sqlx::Error),
    #[error("other error: {0}")]
    Other(anyhow::Error),
}

impl DBError {
    pub fn get_sqlx_error(&self) -> Option<&sqlx::Error> {
        match self {
            Self::Sqlx(err) => Some(err),
            _ => None,
        }
    }

    pub fn is_database_error(&self) -> bool {
        if matches!(self, Self::Sqlx(_)) {
            return matches!(self.get_sqlx_error().unwrap(), sqlx::Error::Database(_));
        }
        false
    }

}
