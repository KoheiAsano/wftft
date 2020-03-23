use super::db;
use super::models::{Article, NewArticle, NewUser, RawArticle, RawUser, User};
use super::Pool;
use actix_web::{web, HttpResponse, Result};
use diesel::prelude::*;
use failure::Error;
// GET
pub async fn handle_get_all_users(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    use crate::schema::users::dsl;
    let conn = pool.get().expect("Failed to get connection from Pool");
    let all_users = dsl::users
        .load::<User>(&conn)
        .expect("Error during loading users");

    Ok(HttpResponse::Ok().json(all_users))
}

pub async fn handle_get_user_by_id(
    pool: web::Data<Pool>,
    uid: web::Path<(i64,)>,
) -> Result<HttpResponse, Error> {
    use crate::schema::users::dsl;
    let conn = pool.get().expect("Failed to get connection from Pool");
    let user = dsl::users
        .filter(dsl::id.eq(uid.0))
        .load::<User>(&conn)
        .expect("Error during loading user by Id");

    Ok(HttpResponse::Ok().json(user))
}

pub async fn handle_get_all_articles(pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    use crate::schema::articles::dsl;
    let conn = pool.get().expect("Failed to get connection from Pool");
    let all_articles = dsl::articles
        .load::<Article>(&conn)
        .expect("Error during loading article");

    Ok(HttpResponse::Ok().json(all_articles))
}

pub async fn handle_get_article_by_id(
    pool: web::Data<Pool>,
    aid: web::Path<(i64,)>,
) -> Result<HttpResponse, Error> {
    use crate::schema::articles::dsl;
    let conn = pool.get().expect("Failed to get connection from Pool");
    let article = dsl::articles
        .filter(dsl::id.eq(aid.0))
        .load::<Article>(&conn)
        .expect("Error during loading article");

    Ok(HttpResponse::Ok().json(article))
}

// POST
pub async fn handle_post_user(
    pool: web::Data<Pool>,
    raw: web::Json<RawUser>,
) -> Result<HttpResponse, Error> {
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

pub async fn handle_post_article<'a>(
    pool: web::Data<Pool>,
    raw: web::Json<RawArticle>,
) -> Result<HttpResponse, Error> {
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
