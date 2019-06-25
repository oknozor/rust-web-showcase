#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde;
use actix_web::{
    http, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};
use model::users::{NewUserDto, UserDto};
use movie_night_db::get_conn;
use movie_night_db::init_pool;
use movie_night_db::users::*;
use movie_night_db::Pool;

pub mod model;

fn user_by_id(pool: web::Data<Pool>, user_id: web::Path<i32>) -> HttpResponse {
    movie_night_db::users::find_by_id(*user_id.as_ref(), pool.get_ref());
    HttpResponse::Ok().json(UserDto {
        id: *user_id.as_ref(),
        nickname: "bob".to_string(),
        email: "bob@bob.com".to_string(),
    }) // <- send response
}

/// This handler uses json extractor
fn post_user(
    pool: web::Data<Pool>,
    new_user: web::Json<NewUserDto>,
) -> Result<HttpResponse, Error> {
    let new_db_user = &new_user.into_inner().into();
    let response = movie_night_db::users::insert(new_db_user, pool.get_ref());
    Ok(HttpResponse::Ok().json(UserDto::from(response))) // <- send response
}


fn main() -> std::io::Result<()> {

    let pool = init_pool("postgres://movie_night:password@localhost/movie_night")
        .expect("unable to connect to the database");

    let app = move || {
        App::new()
            .data(pool.clone())
            .service(web::resource("/users/{id}").route(web::get().to_async(user_by_id)))
            .service(web::resource("/users").route(web::post().to_async(post_user)))
    };

    HttpServer::new(app).bind("localhost:8088")?.run()
}
