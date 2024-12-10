// @generated automatically by Diesel CLI.

diesel::table! {
    users (uid) {
        uid -> Uuid,
        login -> Varchar,
        name -> Nullable<Varchar>,
        avatar -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        created_at -> Timestamp,
        admin -> Bool,
        github_id -> Nullable<Int8>,
        casdoor_id -> Nullable<Varchar>,
    }
}
