#[macro_use]
extern crate serde;
extern crate serde_json;
use actix_web::{web, App, Error, HttpResponse, HttpServer};
use model::users::{NewUserDto, UserDto};
use movie_night_db::init_pool;
use movie_night_db::Pool;

pub mod model;

fn user_by_id(pool: web::Data<Pool>, user_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let user = movie_night_db::users::find_by_id(*user_id.as_ref(), pool.get_ref());
    Ok(HttpResponse::Ok().json(UserDto::from(user)))
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
