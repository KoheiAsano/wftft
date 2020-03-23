use super::models::{Article, NewArticle, NewUser, User};
use diesel::insert_into;
use diesel::prelude::*;
use diesel::result::QueryResult;

pub fn all_users(conn: &PgConnection) -> QueryResult<Vec<User>> {
    use crate::schema::users::dsl;
    dsl::users.load::<User>(conn)
}
pub fn user_by_id(conn: &PgConnection, uid: i64) -> QueryResult<Vec<User>> {
    use crate::schema::users::dsl;
    dsl::users.filter(dsl::id.eq(uid)).load::<User>(conn)
}

pub fn all_articles(conn: &PgConnection) -> QueryResult<Vec<Article>> {
    use crate::schema::articles::dsl;
    dsl::articles.load::<Article>(conn)
}
pub fn article_by_id(conn: &PgConnection, aid: i64) -> QueryResult<Vec<Article>> {
    use crate::schema::articles::dsl;
    dsl::articles.filter(dsl::id.eq(aid)).load::<Article>(conn)
}

pub fn insert_article(conn: &PgConnection, new_article: &NewArticle) -> QueryResult<i64> {
    use crate::schema::articles::dsl;
    diesel::insert_into(dsl::articles)
        .values(new_article)
        .returning(dsl::id)
        .get_result(conn)
}

pub fn insert_user(conn: &PgConnection, new_user: &NewUser) -> QueryResult<i64> {
    use crate::schema::users::dsl;
    insert_into(dsl::users)
        .values(new_user)
        .returning(dsl::id)
        .get_result(conn)
}
