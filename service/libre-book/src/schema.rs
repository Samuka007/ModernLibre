// @generated automatically by Diesel CLI.

diesel::table! {
    books (id) {
        id -> Int4,
        cover_url -> Nullable<Varchar>,
        title -> Nullable<Varchar>,
        author -> Nullable<Varchar>,
        description -> Nullable<Text>,
        rating -> Nullable<Float8>,
        added_date -> Date,
    }
}
