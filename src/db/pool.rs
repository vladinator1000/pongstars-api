use dotenv::dotenv;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;

pub async fn get_pool() -> Result<PgPool, sqlx::Error> {
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("no DATABASE_URL env variable found, please set it to something like: postgres://user:password@localhost:5432/db_name"); // TODO: handle errors

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await?;

    Ok(pool)
}