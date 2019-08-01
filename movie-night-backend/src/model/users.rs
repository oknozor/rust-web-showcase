use movie_night_db::users::*;

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

impl Into<NewUser> for NewUserDto {
    fn into(self) -> NewUser {
        NewUser {
            nick: self.nickname,
            email: self.email,
            password_hash: self.password, // TODO: Bcrypt here
        }
    }
}
impl From<User> for UserDto {
    fn from(user: User) -> Self {
        UserDto {
            id: user.id,
            nickname: user.nick,
            email: user.email,
        }
    }
}
