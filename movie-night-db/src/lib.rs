#[macro_use]
extern crate diesel;

use diesel::r2d2::PoolError;
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager, PooledConnection};

/// Just a handy type alias for Postgresql connection pool 
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub mod users;
mod schema;

/// Build the connnection manager and return the connection pool
pub fn init_pool(database_url: &str) -> Result<Pool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn get_conn(pool: &Pool) -> Result<PgPooledConnection, &'static str> {
    pool.get().map_err(|_| "Can't get connection")
}