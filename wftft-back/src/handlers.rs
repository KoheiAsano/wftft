use super::models::{Article, NewArticle, NewUser, User};
use super::Pool;
use actix_web::{web, HttpResponse, Result};
// GET
pub async fn getAllUsers(_pool: web::Data<Pool>) -> Result<HttpResponse> {
    let user = User::new(1, "asako");

    Ok(HttpResponse::Ok().json(user))
}

pub async fn getUserById(_pool: web::Data<Pool>, uid: web::Path<(i32,)>) -> Result<HttpResponse> {
    info!("{:?}", uid);
    let user = User::new(1, "asako");

    Ok(HttpResponse::Ok().json(user))
}

pub async fn getAllArticles(_pool: web::Data<Pool>) -> Result<HttpResponse> {
    let article = Article::new(1, "asako", "turedure");

    Ok(HttpResponse::Ok().json(article))
}

pub async fn getArticleById(
    _pool: web::Data<Pool>,
    aid: web::Path<(i32,)>,
) -> Result<HttpResponse> {
    info!("{:?}", aid.0);
    println!("{}", aid.0);
    let article = Article::new(1, "asako", "turedure");

    Ok(HttpResponse::Ok().json(article))
}

// POST
pub async fn registerUser(
    _pool: web::Data<Pool>,
    newuser: web::Json<NewUser>,
) -> Result<HttpResponse> {
    println!("{:?}", newuser);
    Ok(HttpResponse::Accepted().finish())
}

pub async fn writeArticle<'a>(
    _pool: web::Data<Pool>,
    newarticle: web::Json<NewArticle>,
) -> Result<HttpResponse> {
    info!("{:?}", newarticle);
    println!("{:?}", newarticle);
    // let article = Article::new(1, newarticle)
    Ok(HttpResponse::Accepted().finish())
}
