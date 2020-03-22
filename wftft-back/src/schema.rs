table! {
    articles (id) {
        id -> Int8,
        author -> Varchar,
        created -> Timestamp,
        content -> Varchar,
        published -> Bool,
    }
}

table! {
    users (id) {
        id -> Int8,
        name -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    articles,
    users,
);
