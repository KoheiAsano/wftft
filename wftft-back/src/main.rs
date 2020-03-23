#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use listenfd::ListenFd;
use log::info;
use simple_logger;
use std::env;

mod db;
mod handlers;
mod models;
mod schema;
pub(crate) type Pool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;

pub fn init_pool() -> Pool {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set.");
    let manager = diesel::r2d2::ConnectionManager::<diesel::PgConnection>::new(db_url);
    diesel::r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    match simple_logger::init_with_level(log::Level::Info) {
        Ok(_) => info!("Started!"),
        Err(_) => panic!("logger error"),
    }
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new()
            .data(init_pool())
            // GET
            .route("/api/users", web::get().to(handlers::handle_get_all_users))
            .route(
                "/api/users/{user_id}",
                web::get().to(handlers::handle_get_user_by_id),
            )
            .route(
                "/api/articles",
                web::get().to(handlers::handle_get_all_articles),
            )
            .route(
                "/api/articles/{article_id}",
                web::get().to(handlers::handle_get_article_by_id),
            )
            // POST
            .route("/api/signin", web::post().to(handlers::handle_post_user))
            .route("/api/write", web::post().to(handlers::handle_post_article))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:3000")?
    };

    server.run().await
}
