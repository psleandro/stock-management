use deadpool_diesel::{Manager, Pool};
use diesel::prelude::*;
use diesel::PgConnection;

use crate::infrastructure::db::models::CreateWorkspaceRow;
use crate::infrastructure::db::models::WorkspaceRow;
use crate::infrastructure::db::schema::workspaces;

use crate::models::workspace::{CreateWorkspace, Workspace};

pub struct WorkspaceRepository {
    pub pool: Pool<Manager<PgConnection>>
}

impl WorkspaceRepository {
    pub fn new(pool: Pool<Manager<PgConnection>>) -> Self {
        Self { pool }
    }

    pub async fn create_workspace(&self, workspace_payload: CreateWorkspace) -> Workspace {
        let connection = self.pool.get().await.unwrap();

        let create_workspace_row = CreateWorkspaceRow {
            name: workspace_payload.name,
            owner_id: workspace_payload.owner_id,
        };

       let created_workspace = connection.interact(|conn| {
            diesel::insert_into(workspaces::table)
                .values(create_workspace_row)
                .returning(WorkspaceRow::as_returning())
                .get_result::<WorkspaceRow>(conn)
        }).await.unwrap().unwrap();

        Workspace {
            id: created_workspace.id,
			name: created_workspace.name,
            owner_id: created_workspace.id,
			created_at: created_workspace.created_at.to_string(),
			updated_at: created_workspace.updated_at.to_string(),
			deleted_at: created_workspace.deleted_at.map(|d| d.to_string()),
        }
    }
}