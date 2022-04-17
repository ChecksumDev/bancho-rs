#![allow(dead_code, unused_variables)]

use actix_web::{get, App, HttpResponse, HttpServer, Responder, web};
extern crate futures;

mod middlewares;
mod routes;
mod utils;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // HttpServer::new(|| {
    //     App::new()
    //         .route("/", web::post().to(routes::osu::domains::cho::index))
    //         .route(
    //             "/web/bancho_connect.php",
    //             web::get().to(routes::osu::domains::osu::bancho_connect),
    //         )
    //         .route(
    //             "/web/osu-getfriends.php",
    //             web::get().to(routes::osu::domains::osu::get_friends),
    //         )
    //         .route("/web/lastfm.php", web::get().to(routes::osu::domains::osu::lastfm))
    //         .route(
    //             "/web/osu-getseasonal.php",
    //             web::get().to(routes::osu::domains::osu::get_seasonal),
    //         )
    //         .route(
    //             "/web/osu-error.php",
    //             web::get().to(routes::osu::domains::osu::osu_error),
    //         )
    //         .route("/web", web::post().to(routes::osu::domains::cho::index))
    //         .route("/users", web::post().to(routes::osu::domains::cho::index))
    // })
    // .bind("127.0.0.1:7272")?
    // .run()
    // .await
    Ok(())
}
