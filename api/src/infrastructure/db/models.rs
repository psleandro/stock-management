use chrono::{DateTime, Utc};
use diesel::deserialize::FromSqlRow;
use diesel::expression::AsExpression;
use diesel::pg::Pg;
use diesel::prelude::*;
use uuid::Uuid;

use crate::infrastructure::db::schema::sql_types::BaseUnit as SqlBaseUnit;
use crate::infrastructure::db::schema::{
    places, products, stock_movements, suppliers, users, workspaces,
};

#[derive(Queryable, Selectable)]
#[diesel(table_name=users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserRow {
    pub id: i32,
    pub name: Option<String>,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
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
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Insertable)]
#[diesel(table_name=workspaces)]
pub struct CreateWorkspaceRow {
    pub name: Option<String>,
    pub owner_id: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, AsExpression, FromSqlRow)]
#[diesel(sql_type = SqlBaseUnit)]
pub enum BaseUnitModel {
    Unit,
    Milligram,
    Milliliter,
}

impl diesel::serialize::ToSql<SqlBaseUnit, Pg> for BaseUnitModel {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, Pg>,
    ) -> diesel::serialize::Result {
        let s = match self {
            BaseUnitModel::Unit => "unit",
            BaseUnitModel::Milligram => "milligram",
            BaseUnitModel::Milliliter => "milliliter",
        };
        diesel::serialize::ToSql::<diesel::sql_types::Text, Pg>::to_sql(s, out)
    }
}

impl diesel::deserialize::FromSql<SqlBaseUnit, Pg> for BaseUnitModel {
    fn from_sql(pg_value: diesel::pg::PgValue<'_>) -> diesel::deserialize::Result<Self> {
        let str_bytes = pg_value.as_bytes();

        match str_bytes {
            b"unit" => Ok(BaseUnitModel::Unit),
            b"milligram" => Ok(BaseUnitModel::Milligram),
            b"milliliter" => Ok(BaseUnitModel::Milliliter),
            _ => Err(format!("Unknown variant: {:?}", str_bytes).into()),
        }
    }
}

#[derive(Queryable, Selectable, Identifiable, Associations)]
#[diesel(table_name=products)]
#[diesel(belongs_to(WorkspaceRow, foreign_key = workspace_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProductRow {
    pub id: Uuid,
    pub workspace_id: i32,
    pub name: String,
    pub base_unit: BaseUnitModel,
    pub brand: Option<String>,
    pub min_stock: i64,
    pub observation: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Insertable)]
#[diesel(table_name=products)]
pub struct CreateProductRow {
    pub workspace_id: i32,
    pub name: String,
    pub base_unit: BaseUnitModel,
    pub brand: Option<String>,
    pub min_stock: i64,
    pub observation: Option<String>,
}

#[derive(AsChangeset)]
#[diesel(table_name=products)]
pub struct UpdateProductRow {
    pub name: Option<String>,
    pub base_unit: Option<BaseUnitModel>,
    pub brand: Option<String>,
    pub min_stock: Option<i64>,
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
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
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
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
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

#[derive(Queryable, Selectable)]
#[diesel(table_name=stock_movements)]
#[diesel(belongs_to(WorkspaceRow, foreign_key = product_id ))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct StockMovementRow {
    pub id: i32,
    pub movement_date: DateTime<Utc>,
    pub product_id: Uuid,
    pub supplier_id: Option<i32>,
    pub place_id: Option<i32>,
    pub quantity: i32,
    pub unit_cost_in_cents: Option<i32>,
    pub invoice_number: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Insertable)]
#[diesel(table_name=stock_movements)]
pub struct StockMovementEntryRow {
    pub movement_date: DateTime<Utc>,
    pub product_id: Uuid,
    pub supplier_id: Option<i32>,
    pub quantity: i32,
    pub unit_cost_in_cents: Option<i32>,
    pub invoice_number: Option<String>,
    pub notes: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name=stock_movements)]
pub struct StockMovementExitRow {
    pub movement_date: DateTime<Utc>,
    pub product_id: Uuid,
    pub place_id: Option<i32>,
    pub quantity: i32,
    pub notes: Option<String>,
}
