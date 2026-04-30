#[cfg(test)]
mod tests {
    use stock_management_api::{
        db::places_repository::PlacesRepository, models::place::UpdatePlace,
    };

    use crate::common::{
        db::{clean_db, create_test_pool, lock_test_db},
        helpers::{create_place, create_user, create_workspace},
    };

    #[tokio::test]
    async fn should_create_place() {
        let _lock = lock_test_db().await;
        let pool = create_test_pool().await;
        clean_db(pool).await;

        let user = create_user(pool, None).await;
        let workspace = create_workspace(pool, user.id, Some("WS".to_string())).await;

        let created = create_place(pool, workspace.id, "Main place").await;

        assert!(created.id > 0);
        assert_eq!(created.workspace_id.value(), workspace.id.value());
        assert_eq!(created.name, "Main place");
    }

    #[tokio::test]
    async fn should_list_places_from_correct_workspace() {
        let _lock = lock_test_db().await;
        let pool = create_test_pool().await;
        clean_db(pool).await;

        let user_a = create_user(pool, None).await;
        let user_b = create_user(pool, None).await;
        let ws_a = create_workspace(pool, user_a.id, Some("WS A".to_string())).await;
        let ws_b = create_workspace(pool, user_b.id, Some("WS B".to_string())).await;

        create_place(pool, ws_a.id, "Place A").await;
        create_place(pool, ws_b.id, "Place B").await;

        let repo = PlacesRepository::new(pool.clone());
        let result = repo.list_places(ws_a.id, "").await.unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].workspace_id.value(), ws_a.id.value());
        assert_eq!(result[0].name, "Place A");
    }

    #[tokio::test]
    async fn should_get_place_by_id_only_within_workspace() {
        let _lock = lock_test_db().await;
        let pool = create_test_pool().await;
        clean_db(pool).await;

        let user_a = create_user(pool, None).await;
        let user_b = create_user(pool, None).await;
        let ws_a = create_workspace(pool, user_a.id, Some("WS A".to_string())).await;
        let ws_b = create_workspace(pool, user_b.id, Some("WS B".to_string())).await;

        let created = create_place(pool, ws_a.id, "Place A").await;

        let repo = PlacesRepository::new(pool.clone());

        let found_in_a = repo.get_place_by_id(ws_a.id, created.id).await.unwrap();
        assert!(found_in_a.is_some());

        let found_in_b = repo.get_place_by_id(ws_b.id, created.id).await.unwrap();
        assert!(found_in_b.is_none());
    }

    #[tokio::test]
    async fn should_update_place_only_in_workspace() {
        let _lock = lock_test_db().await;
        let pool = create_test_pool().await;
        clean_db(pool).await;

        let user_a = create_user(pool, None).await;
        let user_b = create_user(pool, None).await;
        let ws_a = create_workspace(pool, user_a.id, Some("WS A".to_string())).await;
        let ws_b = create_workspace(pool, user_b.id, Some("WS B".to_string())).await;

        let created = create_place(pool, ws_a.id, "Place A").await;

        let repo = PlacesRepository::new(pool.clone());

        let updated_wrong_ws = repo
            .update_place(
                ws_b.id,
                created.id,
                UpdatePlace {
                    name: Some("Wrong".to_string()),
                },
            )
            .await
            .unwrap();
        assert!(updated_wrong_ws.is_none());

        let updated = repo
            .update_place(
                ws_a.id,
                created.id,
                UpdatePlace {
                    name: Some("Place Updated".to_string()),
                },
            )
            .await
            .unwrap()
            .unwrap();

        assert_eq!(updated.id, created.id);
        assert_eq!(updated.workspace_id.value(), ws_a.id.value());
        assert_eq!(updated.name, "Place Updated");
    }

    #[tokio::test]
    async fn should_delete_place_only_in_workspace_and_hide_it() {
        let _lock = lock_test_db().await;
        let pool = create_test_pool().await;
        clean_db(pool).await;

        let user_a = create_user(pool, None).await;
        let user_b = create_user(pool, None).await;
        let ws_a = create_workspace(pool, user_a.id, Some("WS A".to_string())).await;
        let ws_b = create_workspace(pool, user_b.id, Some("WS B".to_string())).await;

        let created = create_place(pool, ws_a.id, "Place A").await;

        let repo = PlacesRepository::new(pool.clone());

        let deleted_wrong_ws = repo.delete_place(ws_b.id, created.id).await.unwrap();
        assert!(deleted_wrong_ws.is_none());

        let deleted = repo
            .delete_place(ws_a.id, created.id)
            .await
            .unwrap()
            .unwrap();
        assert!(deleted.deleted_at.is_some());

        let list_after = repo.list_places(ws_a.id, "").await.unwrap();
        assert!(list_after.is_empty());

        let get_after = repo.get_place_by_id(ws_a.id, created.id).await.unwrap();
        assert!(get_after.is_none());
    }

    #[tokio::test]
    async fn should_search_place_by_numeric_id_within_workspace() {
        let _lock = lock_test_db().await;
        let pool = create_test_pool().await;
        clean_db(pool).await;

        let user_a = create_user(pool, None).await;
        let user_b = create_user(pool, None).await;
        let ws_a = create_workspace(pool, user_a.id, Some("WS A".to_string())).await;
        let ws_b = create_workspace(pool, user_b.id, Some("WS B".to_string())).await;

        let created = create_place(pool, ws_a.id, "Place A").await;
        create_place(pool, ws_b.id, "Place B").await;

        let repo = PlacesRepository::new(pool.clone());

        let result = repo
            .list_places(ws_a.id, &created.id.to_string())
            .await
            .unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id, created.id);
        assert_eq!(result[0].workspace_id.value(), ws_a.id.value());
    }
}
