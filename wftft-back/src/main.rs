use actix_web::{web, App, HttpServer};
use listenfd::ListenFd;
#[macro_use]
extern crate log;
use log::info;
use simple_logger;

mod handlers;
mod models;

#[derive(Clone)]
pub struct Pool {}

impl Pool {
    pub fn new() -> Pool {
        Pool {}
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // ここは?が使えないどうして
    match simple_logger::init_with_level(log::Level::Info) {
        Ok(_) => info!("Started!"),
        Err(_) => panic!("logger error"),
    }
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new()
            .data(Pool::new())
            .route("/api/users", web::get().to(handlers::get_all_users))
            .route(
                "/api/users/{user_id}",
                web::get().to(handlers::get_user_by_id),
            )
            .route("/api/articles", web::get().to(handlers::get_all_articles))
            .route(
                "/api/articles/{article_id}",
                web::get().to(handlers::get_article_by_id),
            )
            .route("/api/signin", web::post().to(handlers::register_user))
            .route("/api/write", web::post().to(handlers::write_article))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:3000")?
    };

    server.run().await
}
