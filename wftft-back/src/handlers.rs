use super::models::{Article, RawArticle, RawUser, User};
use super::Pool;
use actix_web::{web, HttpResponse, Result};
// GET
pub async fn get_all_users(_pool: web::Data<Pool>) -> Result<HttpResponse> {
    let user = User::new(1, "asako");

    Ok(HttpResponse::Ok().json(user))
}

pub async fn get_user_by_id(
    _pool: web::Data<Pool>,
    uid: web::Path<(i32,)>,
) -> Result<HttpResponse> {
    info!("{:?}", uid);
    let user = User::new(1, "asako");

    Ok(HttpResponse::Ok().json(user))
}

pub async fn get_all_articles(_pool: web::Data<Pool>) -> Result<HttpResponse> {
    let article = Article::new(1, "asako", "turedure");

    Ok(HttpResponse::Ok().json(article))
}

pub async fn get_article_by_id(
    _pool: web::Data<Pool>,
    aid: web::Path<(i32,)>,
) -> Result<HttpResponse> {
    info!("{:?}", aid.0);
    println!("{}", aid.0);
    let article = Article::new(1, "asako", "turedure");

    Ok(HttpResponse::Ok().json(article))
}

// POST
pub async fn register_user(
    _pool: web::Data<Pool>,
    newuser: web::Json<RawUser>,
) -> Result<HttpResponse> {
    println!("{:?}", newuser);
    Ok(HttpResponse::Accepted().finish())
}

pub async fn write_article<'a>(
    _pool: web::Data<Pool>,
    newarticle: web::Json<RawArticle>,
) -> Result<HttpResponse> {
    info!("{:?}", newarticle);
    println!("{:?}", newarticle);
    Ok(HttpResponse::Accepted().finish())
}
