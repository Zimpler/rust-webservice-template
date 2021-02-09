use crate::config::Config;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub(crate) async fn create_pool(
    config: &Config,
) -> Result<Pool<Postgres>, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(config.postgres.max_pool_connections)
        .connect(&config.postgres.connection_string())
        .await
}
