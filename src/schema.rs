// @generated automatically by Diesel CLI.

diesel::table! {
    docs (id) {
        id -> Int4,
        title -> Varchar,
        url -> Varchar,
        content -> Text,
        doc_type -> Varchar,
        published -> Bool,
    }
}
