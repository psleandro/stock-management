#[cfg(test)]
mod tests {
    use stock_management_api::{
        db::products_repository::ProductsRepository,
        models::product::{BaseUnit, CreateProduct, UpdateProduct},
    };

    use crate::common::{
        db::{clean_db, create_test_pool, lock_test_db},
        helpers::{create_product, create_user, create_workspace},
    };

    #[tokio::test]
    async fn should_create_product() {
        let _lock = lock_test_db().await;
        let pool = create_test_pool().await;
        clean_db(pool).await;

        let user = create_user(pool, None).await;
        let workspace = create_workspace(pool, user.id, Some("Test workspace".to_string())).await;
        let repo = ProductsRepository::new(pool.clone());

        let result = repo
            .create_product(CreateProduct {
                workspace_id: workspace.id,
                name: "Keyboard".to_string(),
                base_unit: BaseUnit::Unit,
                brand: Some("Logi".to_string()),
                min_stock: 10,
                observation: None,
            })
            .await
            .unwrap();

        assert_eq!(result.id.get_version_num(), 7);
        assert_eq!(result.workspace_id.value(), workspace.id.value());
        assert_eq!(result.name, "Keyboard");
        assert_eq!(result.base_unit, BaseUnit::Unit);
        assert_eq!(result.brand, Some("Logi".to_string()));
        assert_eq!(result.min_stock, 10);
        assert_eq!(result.observation, None);
    }

    #[tokio::test]
    async fn should_list_products_from_correct_workspace() {
        let _lock = lock_test_db().await;
        let pool = create_test_pool().await;
        clean_db(pool).await;

        let user_a = create_user(pool, None).await;
        let user_b = create_user(pool, None).await;
        let ws_a = create_workspace(pool, user_a.id, Some("WS A".to_string())).await;
        let ws_b = create_workspace(pool, user_b.id, Some("WS B".to_string())).await;

        create_product(pool, ws_a.id, "Keyboard").await;
        create_product(pool, ws_b.id, "Mouse").await;

        let repo = ProductsRepository::new(pool.clone());

        let result = repo.list_products(ws_a.id, "").await.unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].workspace_id.value(), ws_a.id.value());
        assert_eq!(result[0].name, "Keyboard");
    }

    #[tokio::test]
    async fn should_get_product_by_id_only_within_workspace() {
        let _lock = lock_test_db().await;
        let pool = create_test_pool().await;
        clean_db(pool).await;

        let user_a = create_user(pool, None).await;
        let user_b = create_user(pool, None).await;
        let ws_a = create_workspace(pool, user_a.id, Some("WS A".to_string())).await;
        let ws_b = create_workspace(pool, user_b.id, Some("WS B".to_string())).await;

        let created = create_product(pool, ws_a.id, "Keyboard").await;

        let repo = ProductsRepository::new(pool.clone());

        let found_in_a = repo.get_product_by_id(ws_a.id, created.id).await.unwrap();
        assert!(found_in_a.is_some());

        let found_in_b = repo.get_product_by_id(ws_b.id, created.id).await.unwrap();
        assert!(found_in_b.is_none());
    }

    #[tokio::test]
    async fn should_update_product_only_in_workspace() {
        let _lock = lock_test_db().await;
        let pool = create_test_pool().await;
        clean_db(pool).await;

        let user_a = create_user(pool, None).await;
        let user_b = create_user(pool, None).await;
        let ws_a = create_workspace(pool, user_a.id, Some("WS A".to_string())).await;
        let ws_b = create_workspace(pool, user_b.id, Some("WS B".to_string())).await;

        let created = create_product(pool, ws_a.id, "Keyboard").await;

        let repo = ProductsRepository::new(pool.clone());

        let updated_wrong_ws = repo
            .update_product(
                ws_b.id,
                created.id,
                UpdateProduct {
                    name: Some("New name".to_string()),
                    base_unit: None,
                    brand: None,
                    min_stock: None,
                    observation: None,
                },
            )
            .await
            .unwrap();
        assert!(updated_wrong_ws.is_none());

        let updated = repo
            .update_product(
                ws_a.id,
                created.id,
                UpdateProduct {
                    name: Some("Mechanical Keyboard".to_string()),
                    base_unit: Some(BaseUnit::Milliliter),
                    brand: Some("KeyCo".to_string()),
                    min_stock: Some(42),
                    observation: Some("updated".to_string()),
                },
            )
            .await
            .unwrap()
            .unwrap();

        assert_eq!(updated.id, created.id);
        assert_eq!(updated.workspace_id.value(), ws_a.id.value());
        assert_eq!(updated.name, "Mechanical Keyboard");
        assert_eq!(updated.base_unit, BaseUnit::Milliliter);
        assert_eq!(updated.brand, Some("KeyCo".to_string()));
        assert_eq!(updated.min_stock, 42);
        assert_eq!(updated.observation, Some("updated".to_string()));
    }

    #[tokio::test]
    async fn should_delete_product_only_in_workspace_and_hide_it() {
        let _lock = lock_test_db().await;
        let pool = create_test_pool().await;
        clean_db(pool).await;

        let user_a = create_user(pool, None).await;
        let user_b = create_user(pool, None).await;
        let ws_a = create_workspace(pool, user_a.id, Some("WS A".to_string())).await;
        let ws_b = create_workspace(pool, user_b.id, Some("WS B".to_string())).await;

        let created = create_product(pool, ws_a.id, "Keyboard").await;
        let repo = ProductsRepository::new(pool.clone());

        let deleted_wrong_ws = repo.delete_product(ws_b.id, created.id).await.unwrap();
        assert!(deleted_wrong_ws.is_none());

        let deleted = repo
            .delete_product(ws_a.id, created.id)
            .await
            .unwrap()
            .unwrap();
        assert!(deleted.deleted_at.is_some());

        let list_after = repo.list_products(ws_a.id, "").await.unwrap();
        assert!(list_after.is_empty());

        let get_after = repo.get_product_by_id(ws_a.id, created.id).await.unwrap();
        assert!(get_after.is_none());
    }

    #[tokio::test]
    async fn should_search_product_by_numeric_id_within_workspace() {
        let _lock = lock_test_db().await;
        let pool = create_test_pool().await;
        clean_db(pool).await;

        let user_a = create_user(pool, None).await;
        let user_b = create_user(pool, None).await;
        let ws_a = create_workspace(pool, user_a.id, Some("WS A".to_string())).await;
        let ws_b = create_workspace(pool, user_b.id, Some("WS B".to_string())).await;

        let created = create_product(pool, ws_a.id, "Keyboard").await;
        create_product(pool, ws_b.id, "Mouse").await;

        let repo = ProductsRepository::new(pool.clone());

        let result = repo
            .list_products(ws_a.id, &created.id.to_string())
            .await
            .unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id, created.id);
        assert_eq!(result[0].workspace_id.value(), ws_a.id.value());
    }
}
