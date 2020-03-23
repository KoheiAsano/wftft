use super::db;
use super::models::{NewArticle, NewUser, RawArticle, RawUser};
use super::Pool;
use actix_web::{web, HttpResponse, Result};
use failure::Error;
// GET
pub async fn handle_get_all_users(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Failed to get connection from Pool");
    let all_users = db::all_users(&conn)?;
    Ok(HttpResponse::Ok().json(all_users))
}

pub async fn handle_get_user_by_id(
    pool: web::Data<Pool>,
    uid: web::Path<(i64,)>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Failed to get connection from Pool");
    let user = db::user_by_id(&conn, uid.0)?;

    Ok(HttpResponse::Ok().json(user))
}

pub async fn handle_get_all_articles(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Failed to get connection from Pool");
    let all_articles = db::all_articles(&conn)?;

    Ok(HttpResponse::Ok().json(all_articles))
}

pub async fn handle_get_article_by_id(
    pool: web::Data<Pool>,
    aid: web::Path<(i64,)>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Failed to get connection from Pool");
    let article = db::article_by_id(&conn, aid.0)?;
    Ok(HttpResponse::Ok().json(article))
}

// POST
pub async fn handle_post_user(
    pool: web::Data<Pool>,
    raw: web::Json<RawUser>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Failed to get connection from Pool");
    let new = NewUser {
        name: raw.name.clone(),
    };
    db::insert_user(&conn, &new)?;
    info!("post new user:{:?}", new);
    Ok(HttpResponse::Accepted().finish())
}

pub async fn handle_post_article<'a>(
    pool: web::Data<Pool>,
    raw: web::Json<RawArticle>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("Failed to get connection from Pool");
    let new = NewArticle {
        author: raw.author.clone(),
        created: raw.created.unwrap_or(chrono::Utc::now()).naive_utc(),
        content: raw.content.clone(),
        published: raw.published,
    };
    db::insert_article(&conn, &new)?;
    info!("post new article:{:?}", new);
    Ok(HttpResponse::Accepted().finish())
}
