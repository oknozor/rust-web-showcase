#[macro_use]
extern crate diesel;

use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};

/// Just a handy type alias for Postgresql connection pool 
type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

mod users;
mod schema;