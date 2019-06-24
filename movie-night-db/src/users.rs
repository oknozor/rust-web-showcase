
use super::schema::users;
use super::schema::users::dsl::*;
use diesel::prelude::*;

use super::Pool;

#[derive(Identifiable, Queryable, PartialEq, Debug)]
pub struct User {
        pub id: i32,
        pub nick: String,
        pub email: String,
        pub password_hash: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
        pub nick: &'a str,
        pub email: &'a str,
        pub password_hash: String,
}

pub fn find_by_id(user_id: i32, pool: Pool) -> User {
        let conn: &PgConnection = &pool.get().expect("Unable to connect to the database");
        users.find(user_id)
                .get_result::<User>(conn)
                .unwrap_or_else(|_| panic!("User with id {} not found", user_id))
}

pub fn all(pool: Pool) -> Vec<User> {
        let conn: &PgConnection = &pool.get().expect("Unable to connect to the database");
        users.load::<User>(conn).expect("Error loading users")
}

pub fn insert(new_user: &NewUser, pool: Pool) -> User {
        let conn: &PgConnection = &pool.get().expect("Unable to connect to the database");
        diesel::insert_into(users::table)
                .values(new_user)
                .get_result(conn)
                .expect("Error saving new users")
}
