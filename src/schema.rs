// @generated automatically by Diesel CLI.

diesel::table! {
    post (id) {
        id -> Int4,
        title -> Varchar,
        slug -> Varchar,
        body -> Text,
    }
}
