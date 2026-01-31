use deadpool_diesel::{Manager, Pool};
use diesel::PgConnection;
use diesel::prelude::*;

use crate::errors::InfrastructureError;
use crate::infrastructure::db::models::{CreateWorkspaceRow, WorkspaceRow};
use crate::infrastructure::db::schema::workspaces;
use crate::models::ids::{UserId, WorkspaceId};
use crate::models::workspace::{CreateWorkspace, Workspace};

pub struct WorkspaceRepository {
    pub pool: Pool<Manager<PgConnection>>,
}

impl WorkspaceRepository {
    pub fn new(pool: Pool<Manager<PgConnection>>) -> Self {
        Self { pool }
    }

    pub async fn create_workspace(
        &self,
        workspace_payload: CreateWorkspace,
    ) -> Result<Workspace, InfrastructureError> {
        let connection = self
            .pool
            .get()
            .await
            .map_err(|e| InfrastructureError::Unexpected(e.to_string()))?;

        let created_workspace = connection
            .interact(|conn| Self::insert_workspace(conn, workspace_payload))
            .await
            .map_err(|e| InfrastructureError::Unexpected(e.to_string()))?
            .map_err(|e| InfrastructureError::Query(e.to_string()))?;

        Ok(Workspace {
            id: WorkspaceId(created_workspace.id),
            name: created_workspace.name,
            owner_id: UserId(created_workspace.owner_id),
            created_at: created_workspace.created_at.to_string(),
            updated_at: created_workspace.updated_at.to_string(),
            deleted_at: created_workspace.deleted_at.map(|d| d.to_string()),
        })
    }

    pub fn create_workspace_in_tx(
        conn: &mut PgConnection,
        workspace_payload: CreateWorkspace,
    ) -> Result<WorkspaceRow, diesel::result::Error> {
        Self::insert_workspace(conn, workspace_payload)
    }

    fn insert_workspace(
        conn: &mut PgConnection,
        workspace_payload: CreateWorkspace,
    ) -> Result<WorkspaceRow, diesel::result::Error> {
        let create_workspace_row = CreateWorkspaceRow {
            name: workspace_payload.name,
            owner_id: workspace_payload.owner_id.value(),
        };

        diesel::insert_into(workspaces::table)
            .values(create_workspace_row)
            .returning(WorkspaceRow::as_returning())
            .get_result(conn)
    }
}
