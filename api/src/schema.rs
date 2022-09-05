// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Int4,
        email -> Varchar,
        password -> Varchar,
        firstname -> Varchar,
        lastname -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    bankdetails (id) {
        id -> Int4,
        account_id -> Int4,
        holder -> Varchar,
        iban -> Varchar,
        bic -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    cardetails (id) {
        id -> Int4,
        account_id -> Int4,
        owner -> Varchar,
        plate -> Varchar,
        registration -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    bankdetails,
    cardetails,
);
