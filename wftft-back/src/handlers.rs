use super::models::{Article, NewArticle, NewUser, RawArticle, RawUser, User};
use super::schema;
use super::Pool;
use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;
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
pub async fn register_user(pool: web::Data<Pool>, raw: web::Json<RawUser>) -> Result<HttpResponse> {
    let conn = pool.get().expect("Failed to get connection from Pool");
    use crate::schema::users::dsl;
    let new = NewUser {
        name: raw.name.clone(),
    };
    // TODO: Error interface
    // get_result
    diesel::insert_into(dsl::users)
        .values(new)
        .returning(dsl::id)
        .execute(&conn)
        .expect("insert error");
    Ok(HttpResponse::Accepted().finish())
}

pub async fn write_article<'a>(
    pool: web::Data<Pool>,
    raw: web::Json<RawArticle>,
) -> Result<HttpResponse> {
    let conn = pool.get().expect("Failed to get connection from Pool");
    use crate::schema::articles::dsl;
    let new = NewArticle {
        author: raw.author.clone(),
        created: raw.created.unwrap_or(chrono::Utc::now()).naive_utc(),
        content: raw.content.clone(),
        published: raw.published,
    };
    // TODO: Error interface
    // get_result
    diesel::insert_into(dsl::articles)
        .values(new)
        .returning(dsl::id)
        .execute(&conn)
        .expect("insert error");
    Ok(HttpResponse::Accepted().finish())
}
