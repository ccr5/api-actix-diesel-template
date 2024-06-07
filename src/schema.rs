// @generated automatically by Diesel CLI.

diesel::table! {
    cash_in (id) {
        id -> Int4,
        amount -> Numeric,
        #[max_length = 300]
        description -> Varchar,
        date -> Timestamp,
        #[max_length = 50]
        status -> Varchar,
    }
}

diesel::table! {
    cash_out (id) {
        id -> Int4,
        amount -> Numeric,
        #[max_length = 255]
        description -> Varchar,
        #[max_length = 255]
        cash_out_type -> Varchar,
        installment_number -> Nullable<Int4>,
        total_installments -> Nullable<Int4>,
        date -> Date,
        #[max_length = 255]
        status -> Varchar,
        observation -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    cash_in,
    cash_out,
);
