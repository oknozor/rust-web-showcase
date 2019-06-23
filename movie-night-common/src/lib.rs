extern crate serde;
use serde::{Deserialize, Serialize};

/// Dto representing a Movie, data are currently fetched via the movie database
#[derive(Serialize, Deserialize, Debug)]
pub struct MovieDto {
    pub id: i32,
    pub tmdb_id: i32,
    pub title: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
}

/// Result for tmdb movie search
#[derive(Serialize, Deserialize, Debug)]
pub struct SearchResultMovieDto {
    pub tmdb_id: u64,
    pub title: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
}

/// Add a new movie
#[derive(Serialize, Deserialize, Debug)]
pub struct NewMovieDto {
    pub tmdb_id: i32,
}

/// Represent all the movies bookmarked by a given user
#[derive(Serialize, Deserialize, Debug)]
pub struct UserMoviesDto {
    pub user: UserDto,
    pub movies: Vec<MovieDto>,
}

/// Add a movie to user bookmarks given a user id and a moviedb id 
#[derive(Serialize, Deserialize, Debug)]
pub struct NewUserMovieDto {
    pub user_id: i32,
    pub tmdb_id: i32,
}

/// User without its password
#[derive(Serialize, Deserialize, Debug)]
pub struct UserDto {
    pub id: Option<i32>,
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