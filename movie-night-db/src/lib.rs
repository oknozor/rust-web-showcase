#[macro_use]
extern crate diesel;

use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::PoolError;
use diesel::r2d2::PooledConnection;
use diesel::result::Error as DieselError;
use std::error::Error;
use std::ops::Deref;
use users::*;

/// Just a handy type alias for Postgresql connection pool
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

mod schema;
pub mod users;
/// Build the connnection manager and return the connection pool
pub fn init_pool(database_url: &str) -> Result<Pool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub fn get_conn(pool: &Pool) -> Result<PgPooledConnection, &'static str> {
    pool.get().map_err(|_| "Can't get connection")
}

pub fn all_users(pool: &Pool) -> Result<Vec<User>, String> {
    User::all(get_conn(pool)?.deref()).map_err(map_db_err)
}

pub fn user_by_id(user_id: i32, pool: &Pool) -> Result<User, String> {
    User::by_id(user_id, get_conn(pool)?.deref()).map_err(map_db_err)
}

fn map_db_err(err: DieselError) -> String {
    match err {
        DieselError::DatabaseError(_kind, _info) => String::from(_info.message()),
        DieselError::NotFound => String::from("Resource not found in database"),
        _ => String::from("Unkown database error"),
    }
}
