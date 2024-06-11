// @generated automatically by Diesel CLI.

diesel::table! {
    cash_in (id) {
        id -> Int4,
        #[max_length = 300]
        description -> Varchar,
        #[max_length = 50]
        status -> Varchar,
    }
}
