use chrono::NaiveDateTime;
use serde::Serialize;

use crate::{infrastructure::db::models::PlaceRow, models::ids::WorkspaceId};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Place {
    pub id: i32,

    #[serde(skip_serializing)]
    pub workspace_id: WorkspaceId,

    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<NaiveDateTime>,
}

impl From<PlaceRow> for Place {
    fn from(row: PlaceRow) -> Self {
        Place {
            id: row.id,
            workspace_id: WorkspaceId(row.workspace_id),
            name: row.name,
            created_at: row.created_at,
            updated_at: row.updated_at,
            deleted_at: row.deleted_at,
        }
    }
}

pub struct CreatePlace {
    pub workspace_id: WorkspaceId,
    pub name: String,
}

pub struct UpdatePlace {
    pub name: Option<String>,
}
