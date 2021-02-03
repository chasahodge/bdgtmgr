table! {
    accounts (id) {
        id -> Int4,
        account_name -> Varchar,
        account_holder -> Text,
        balance -> Numeric,
    }
}

table! {
    transactions (id) {
        id -> Int4,
        account_name -> Varchar,
        occurrance -> Varchar,
        amount -> Numeric,
    }
}

allow_tables_to_appear_in_same_query!(
    accounts,
    transactions,
);
