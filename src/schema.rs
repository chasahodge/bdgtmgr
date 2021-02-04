table! {
    accounts (id) {
        id -> Int4,
        account_name -> Varchar,
        account_number -> Varchar,
        account_holder -> Varchar,
        balance -> Float,
    }
}

table! {
    transactions (id) {
        id -> Int4,
        account_name -> Varchar,
        account_number -> Varchar,
        locale -> Varchar,
        occurrance -> Varchar,
        amount -> Float,
    }
}

allow_tables_to_appear_in_same_query!(
    accounts,
    transactions,
);
