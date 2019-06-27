#[macro_use]
extern crate serde;
extern crate serde_json;
use futures::future::{err, Future, IntoFuture};
use actix_web::{web, App, Error, HttpResponse, HttpServer};
use model::users::{NewUserDto, UserDto};
use movie_night_db::init_pool;
use movie_night_db::Pool;

pub mod model;

#[derive(Serialize, Debug)]
pub struct JsonError { 
    message: String,
}

fn user_by_id(pool: web::Data<Pool>, user_id: web::Path<i32>) -> impl Future<Item=HttpResponse, Error=Error> {
    web::block( move || movie_night_db::user_by_id(*user_id.as_ref(), &pool.get_ref()))
        .then( |resp| {
            match resp {
                Ok(user) => Ok(HttpResponse::Ok().json(UserDto::from(user))), 
                Err(err) => Ok(HttpResponse::InternalServerError().json(JsonError {message: err.to_string()}))
            }
        })
}

fn post_user(
    pool: web::Data<Pool>,
    new_user: web::Json<NewUserDto>,
) { unimplemented!()}

fn delete(pool: web::Data<Pool>, user_id: web::Path<i32>) -> Result<HttpResponse, Error> {unimplemented!()}

fn search_user(pool: web::Data<Pool>, search_query: &str) -> Result<HttpResponse, Error> {unimplemented!()}


fn main() -> std::io::Result<()> {

    let pool = init_pool("postgres://movie_night:password@localhost/movie_night")
        .expect("unable to connect to the database");

    let app = move || {
        App::new()
            .data(pool.clone())
            .service(web::resource("/users/{id}").route(web::get().to_async(user_by_id)))
    };

    HttpServer::new(app).bind("localhost:8088")?.run()
}
