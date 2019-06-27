#[macro_use]
extern crate diesel;


use diesel::result::Error as DieselError;
use std::ops::Deref;
use diesel::r2d2::PoolError;
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager, PooledConnection};
use users::*; 

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

pub fn user_by_id(user_id: i32, pool: &Pool) -> Result<User, String> {
    User::by_id(user_id, get_conn(pool)?.deref()).map_err(|err| match err {
        //TODO: map this to detailed errors and some how map these error to HTTP response codes in the backend
        DieselError::DatabaseError(_kind, _info) => String::from(_info.message()),
        DieselError::NotFound => String::from("User not found in database"),
        _ => String::from("unkown error")
    })
}