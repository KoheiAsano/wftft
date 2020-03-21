use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// 返り値で使う記事型
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
