//!
#![warn(missing_debug_implementations, missing_docs, rust_2018_idioms)]

use actix_web::{App, HttpServer};
use chrono::Utc;
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;

mod config;
mod status;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    debug!("Loading configuration");
    let config = config::get_config().expect("failed to read configuration");

    debug!("Initializing Postgres database");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.postgres.connection_string())
        .await
        .unwrap();

    // // Example statement.
    // sqlx::query!(
    //     "insert into persons (id, name, created_at) \
    //      values ($1, $2, $3)",
    //     Uuid::new_v4(),
    //     "hello, world",
    //     Utc::now().naive_utc(),
    // )
    // .execute(&pool)
    // .await.unwrap();

    // // Example query.
    // let persons = sqlx::query!("select * from persons")
    //     .fetch_all(&pool)
    //     .await
    //     .unwrap();
    // dbg!(persons);

    HttpServer::new(|| App::new().service(status::status))
        .bind(format!("127.0.0.1:{}", config.server.port))?
        .run()
        .await
}
