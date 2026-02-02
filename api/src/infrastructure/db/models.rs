use chrono::NaiveDateTime;
use diesel::prelude::*;

use crate::infrastructure::db::schema::{places, products, suppliers, users, workspaces};

#[derive(Queryable, Selectable)]
#[diesel(table_name=users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserRow {
    pub id: i32,
    pub name: Option<String>,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name=users)]
pub struct CreateUserRow {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Queryable, Selectable, Associations)]
#[diesel(table_name=workspaces)]
#[diesel(belongs_to(UserRow, foreign_key = owner_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct WorkspaceRow {
    pub id: i32,
    pub name: Option<String>,
    pub owner_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name=workspaces)]
pub struct CreateWorkspaceRow {
    pub name: Option<String>,
    pub owner_id: i32,
}

#[derive(Queryable, Selectable, Identifiable, Associations)]
#[diesel(table_name=products)]
#[diesel(belongs_to(WorkspaceRow, foreign_key = workspace_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProductRow {
    pub id: i32,
    pub workspace_id: i32,
    pub name: String,
    pub unit: Option<String>,
    pub brand: Option<String>,
    pub min_stock: i32,
    pub observation: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name=products)]
pub struct CreateProductRow {
    pub workspace_id: i32,
    pub name: String,
    pub unit: Option<String>,
    pub brand: Option<String>,
    pub min_stock: i32,
    pub observation: Option<String>,
}

#[derive(AsChangeset)]
#[diesel(table_name=products)]
pub struct UpdateProductRow {
    pub name: Option<String>,
    pub unit: Option<String>,
    pub brand: Option<String>,
    pub min_stock: Option<i32>,
    pub observation: Option<String>,
}

#[derive(Queryable, Selectable, Identifiable, Associations)]
#[diesel(table_name=places)]
#[diesel(belongs_to(WorkspaceRow, foreign_key = workspace_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PlaceRow {
    pub id: i32,
    pub workspace_id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name=places)]
pub struct CreatePlaceRow {
    pub name: String,
    pub workspace_id: i32,
}

#[derive(AsChangeset)]
#[diesel(table_name=places)]
pub struct UpdatePlaceRow {
    pub name: Option<String>,
}

#[derive(Queryable, Selectable, Identifiable, Associations)]
#[diesel(table_name=suppliers)]
#[diesel(belongs_to(WorkspaceRow, foreign_key = workspace_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SupplierRow {
    pub id: i32,
    pub workspace_id: i32,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name=suppliers)]
pub struct CreateSupplierRow {
    pub name: String,
    pub workspace_id: i32,
}

#[derive(AsChangeset)]
#[diesel(table_name=suppliers)]
pub struct UpdateSupplierRow {
    pub name: Option<String>,
}
