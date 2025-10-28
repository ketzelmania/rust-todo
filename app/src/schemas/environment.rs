use std::{convert::Infallible, env::Args, error::Error};

use sqlx::{PgPool, Pool};
use warp::Filter;

#[derive(Debug, Clone)]
pub struct Environment {
    db_pool: PgPool,
}

impl Environment {
    pub async fn new(database_url: &str) -> Result<Self, Box<dyn Error>> {
        let db_pool = PgPool::connect(database_url).await?;

        Ok(Self { db_pool })
    }

    pub async fn from_pool(pool: PgPool) -> Result<Self, Box<dyn Error>> {
        Ok(Self { db_pool: pool })
    }

    pub fn db(&self) -> &PgPool {
        &self.db_pool
    }
}

pub fn with_env(
    env: Environment,
) -> impl Filter<Extract = (Environment,), Error = Infallible> + Clone {
    warp::any().map(move || env.clone())
}
