
use super::schema::users;
use super::schema::users::dsl::*;
use diesel::prelude::*;

#[derive(Identifiable, Queryable, PartialEq, Debug)]
pub struct User {
        pub id: i32,
        pub nick: String,
        pub email: String,
        pub password_hash: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
        pub nick: String,
        pub email: String,
        pub password_hash: String,
}

impl User {
        pub fn by_id(user_id: i32, conn: &PgConnection) -> QueryResult<User> {
                users.find(user_id).get_result::<User>(conn)
        }

        pub fn all(conn: &PgConnection) -> QueryResult<Vec<User>> {
                users.load::<User>(conn)
        }

        pub fn insert(new_user: &NewUser, conn: &PgConnection) -> QueryResult<User> {
                diesel::insert_into(users::table)
                        .values(new_user)
                        .get_result(conn)
        }
}

