use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use r2d2::{Pool, PooledConnection};
use std::env;

// The Postgres-specific connection pool managing all database connections.
pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;
pub type Connection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn get_pool() -> PostgresPool {
    // TODO: pass the connection URL into this function rather than extracting
    // it from the environment within this function
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("no DATABASE_URL env variable found, please create one"); // TODO: handle errors
    let manager = ConnectionManager::<PgConnection>::new(url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("could not build connection pool") // TODO: handle errors
}
