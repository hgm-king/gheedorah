table! {
    shopify_integrations (id) {
        id -> Int4,
        shop -> Varchar,
        nonce -> Varchar,
        access_token -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        active -> Bool,
    }
}
