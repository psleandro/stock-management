use chrono::NaiveDateTime;
use serde::Serialize;

use crate::infrastructure::db::models::ProductRow;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Product {
    pub id: i32,

    #[serde(skip_serializing)]
    pub workspace_id: i32,

    pub name: String,
    pub unit: Option<String>,
    pub brand: Option<String>,
    pub min_stock: i32,
    pub observation: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<NaiveDateTime>,
}

impl From<ProductRow> for Product {
    fn from(row: ProductRow) -> Self {
        Product {
            id: row.id,
            workspace_id: row.workspace_id,
            name: row.name,
            unit: row.unit,
            brand: row.brand,
            min_stock: row.min_stock,
            observation: row.observation,
            created_at: row.created_at,
            updated_at: row.updated_at,
            deleted_at: row.deleted_at,
        }
    }
}

pub struct CreateProduct {
    pub workspace_id: i32,
    pub name: String,
    pub unit: Option<String>,
    pub brand: Option<String>,
    pub min_stock: i32,
    pub observation: Option<String>,
}

pub struct UpdateProduct {
    pub name: Option<String>,
    pub unit: Option<String>,
    pub brand: Option<String>,
    pub min_stock: Option<i32>,
    pub observation: Option<String>,
}
