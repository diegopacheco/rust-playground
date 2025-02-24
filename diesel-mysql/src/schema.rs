// @generated automatically by Diesel CLI.

diesel::table! {
    person (id) {
        id -> Integer,
        first_name -> Varchar,
        last_name -> Varchar,
        created_at -> Timestamp,
    }
}