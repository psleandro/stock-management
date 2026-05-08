// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "base_unit"))]
    pub struct BaseUnit;
}

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
    use diesel::sql_types::*;
    use super::sql_types::BaseUnit;

    products (id) {
        id -> Uuid,
        workspace_id -> Int4,
        name -> Text,
        base_unit -> BaseUnit,
        brand -> Nullable<Text>,
        min_stock -> Int8,
        observation -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    stock_movements (id) {
        id -> Int4,
        movement_date -> Timestamp,
        product_id -> Uuid,
        supplier_id -> Nullable<Int4>,
        place_id -> Nullable<Int4>,
        quantity -> Int4,
        unit_cost_in_cents -> Nullable<Int4>,
        invoice_number -> Nullable<Text>,
        notes -> Nullable<Text>,
        created_at -> Timestamp,
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
diesel::joinable!(stock_movements -> places (place_id));
diesel::joinable!(stock_movements -> products (product_id));
diesel::joinable!(stock_movements -> suppliers (supplier_id));
diesel::joinable!(suppliers -> workspaces (workspace_id));
diesel::joinable!(workspaces -> users (owner_id));

diesel::allow_tables_to_appear_in_same_query!(
    places,
    products,
    stock_movements,
    suppliers,
    users,
    workspaces,
);
