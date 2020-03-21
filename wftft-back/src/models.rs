use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// for response
#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct Article {
    pub id: i32,
    pub author: String,
    pub created: DateTime<Utc>,
    pub content: String,
    pub published: bool,
}
impl<'a> Article {
    pub fn new(id: i32, author: &'a str, content: &'a str) -> Article {
        Article {
            id,
            author: String::from(author),
            created: Utc::now(),
            content: String::from(content),
            published: false,
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
}
impl<'a> User {
    pub fn new(id: i32, name: &'a str) -> User {
        User {
            id,
            name: String::from(name),
        }
    }
}

// for requeset
#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct NewArticle {
    pub author: String,
    pub created: Option<DateTime<Utc>>,
    pub content: String,
    pub published: bool,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct NewUser {
    pub id: i32,
    pub name: String,
}
