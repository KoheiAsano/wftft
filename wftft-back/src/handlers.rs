use super::models;
use actix_web::{web, HttpResponse, Result};
// GET
pub async fn getAllUsers() -> Result<HttpResponse> {
    let article = models::Article::new(1, "asako", "turedure");

    Ok(HttpResponse::Ok().json(article))
}

pub async fn getUserById(info: web::Path<(i32,)>) -> Result<HttpResponse> {
    debug!("{:?}", info);
    let article = models::Article::new(1, "asako", "turedure");

    Ok(HttpResponse::Ok().json(article))
}

pub async fn getAllArticles() -> Result<HttpResponse> {
    let article = models::Article::new(1, "asako", "turedure");

    Ok(HttpResponse::Ok().json(article))
}

pub async fn getArticleById() -> Result<HttpResponse> {
    let article = models::Article::new(1, "asako", "turedure");

    Ok(HttpResponse::Ok().json(article))
}

pub async fn getArticleByUserId() -> Result<HttpResponse> {
    let article = models::Article::new(1, "asako", "turedure");

    Ok(HttpResponse::Ok().json(article))
}

// POST
pub async fn registerUser() -> Result<HttpResponse> {
    Ok(HttpResponse::Accepted().finish())
}

pub async fn writeArticle() -> Result<HttpResponse> {
    Ok(HttpResponse::Accepted().finish())
}
