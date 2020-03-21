use actix_web::{web, App, HttpServer};
use listenfd::ListenFd;
#[macro_use]
extern crate log;
use log::info;
use simple_logger;

mod handlers;
mod models;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    simple_logger::init_with_level(log::Level::Info);
    info!("Started!");
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new()
            .route("/api/users", web::get().to(handlers::getAllUsers))
            .route("/api/users/{user_id}", web::get().to(handlers::getAllUsers))
            .route("/api/articles", web::get().to(handlers::getAllArticles))
            .route(
                "/api/articles/{article_id}",
                web::get().to(handlers::getAllArticles),
            )
            .route("/api/signin", web::post().to(handlers::registerUser))
            .route("/api/write", web::post().to(handlers::writeArticle))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:3000")?
    };

    server.run().await
}
