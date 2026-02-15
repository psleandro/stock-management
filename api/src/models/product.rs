use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::{
    infrastructure::db::models::{BaseUnitModel, ProductRow},
    models::ids::WorkspaceId,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum BaseUnit {
    Unit,
    Milligram,
    Milliliter,
}

impl From<BaseUnitModel> for BaseUnit {
    fn from(value: BaseUnitModel) -> Self {
        match value {
            BaseUnitModel::Unit => BaseUnit::Unit,
            BaseUnitModel::Milligram => BaseUnit::Milligram,
            BaseUnitModel::Milliliter => BaseUnit::Milliliter,
        }
    }
}

impl From<BaseUnit> for BaseUnitModel {
    fn from(value: BaseUnit) -> Self {
        match value {
            BaseUnit::Unit => BaseUnitModel::Unit,
            BaseUnit::Milligram => BaseUnitModel::Milligram,
            BaseUnit::Milliliter => BaseUnitModel::Milliliter,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Product {
    pub id: i32,

    #[serde(skip_serializing)]
    pub workspace_id: WorkspaceId,

    pub name: String,
    pub base_unit: BaseUnit,
    pub brand: Option<String>,
    pub min_stock: i64,
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
            workspace_id: WorkspaceId(row.workspace_id),
            name: row.name,
            base_unit: row.base_unit.into(),
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
    pub workspace_id: WorkspaceId,
    pub name: String,
    pub base_unit: BaseUnit,
    pub brand: Option<String>,
    pub min_stock: i64,
    pub observation: Option<String>,
}

pub struct UpdateProduct {
    pub name: Option<String>,
    pub base_unit: Option<BaseUnit>,
    pub brand: Option<String>,
    pub min_stock: Option<i64>,
    pub observation: Option<String>,
}

#[derive(Serialize)]
pub struct Stock {
    pub min: i64,
    pub current: i64,
}
