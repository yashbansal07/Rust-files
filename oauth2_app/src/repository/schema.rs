// @generated automatically by Diesel CLI.

diesel::table! {
    client_info (id) {
        id -> Integer,
        firstname -> Varchar,
        lastname -> Text,
        age -> Integer,
    }
}
