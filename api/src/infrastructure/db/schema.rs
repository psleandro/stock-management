// @generated automatically by Diesel CLI.

diesel::table! {
    places (id) {
        id -> Int4,
        workspace_id -> Int4,
        name -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    products (id) {
        id -> Int4,
        workspace_id -> Int4,
        name -> Text,
        unit -> Nullable<Text>,
        brand -> Nullable<Text>,
        min_stock -> Int4,
        observation -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    suppliers (id) {
        id -> Int4,
        workspace_id -> Int4,
        name -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Nullable<Text>,
        email -> Text,
        password -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    workspaces (id) {
        id -> Int4,
        name -> Nullable<Text>,
        owner_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(places -> workspaces (workspace_id));
diesel::joinable!(products -> workspaces (workspace_id));
diesel::joinable!(suppliers -> workspaces (workspace_id));
diesel::joinable!(workspaces -> users (owner_id));

diesel::allow_tables_to_appear_in_same_query!(places, products, suppliers, users, workspaces,);
