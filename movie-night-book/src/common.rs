extern crate serde;
use serde::{Deserialize, Serialize};


/// User without its password
#[derive(Serialize, Deserialize, Debug)]
pub struct UserDto {
    pub id: i32,
    pub nickname: String,
    pub email: String,
}

/// Add a new user
#[derive(Serialize, Deserialize, Debug)]
pub struct NewUserDto {
    pub nickname: String,
    pub email: String,
    pub password: String,
}
