#[macro_use]
extern crate serde_derive;
use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use futures::Future;

fn index(req: &HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body("ok")
}

fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::resource("/users")
                    .route(web::post().to(|| HttpResponse::Ok()))
                    .route(web::get().to(|| HttpResponse::Ok()))
                    .route(web::put().to(|| HttpResponse::Ok()))
                    .route(web::patch().to(|| HttpResponse::Ok()))
                    .route(web::delete().to(|| HttpResponse::Ok())),
            )
            .service(
                web::resource("/movies")
                    .route(web::post().to(|| HttpResponse::Ok()))
                    .route(web::get().to(|| HttpResponse::Ok()))
                    .route(web::put().to(|| HttpResponse::Ok()))
                    .route(web::patch().to(|| HttpResponse::Ok()))
                    .route(web::delete().to(|| HttpResponse::Ok())),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
}
