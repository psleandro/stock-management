#[cfg(test)]
mod tests {
    use chrono::Utc;
    use stock_management_api::{
        db::stock_movements_repository::StockMovementsRepository,
        models::stock_movement::{StockMovementEntry, StockMovementExit},
    };

    use crate::common::{
        db::{clean_db, create_test_pool, lock_test_db},
        helpers::{create_place, create_product, create_supplier, create_user, create_workspace},
    };

    #[tokio::test]
    async fn should_create_stock_entry() {
        let _lock = lock_test_db().await;
        let pool = create_test_pool().await;
        clean_db(pool).await;

        let user = create_user(pool, None).await;
        let workspace = create_workspace(pool, user.id, Some("Test workspace".to_string())).await;
        let product = create_product(pool, workspace.id, "Keyboard").await;
        let supplier = create_supplier(pool, workspace.id, "Logitech").await;

        let repo = StockMovementsRepository::new(pool.clone());

        let movement_date = Utc::now();
        let result = repo
            .create_stock_entry(StockMovementEntry {
                movement_date,
                product_id: product.id,
                supplier_id: supplier.id,
                quantity: 10,
                unit_cost_in_cents: 5000,
                invoice_number: "INV-001".to_string(),
                notes: Some("Initial stock".to_string()),
            })
            .await
            .unwrap();

        assert!(result.id > 0);
        assert_eq!(result.product_id, product.id);
        assert_eq!(result.supplier_id, Some(supplier.id));
        assert_eq!(result.quantity, 10);
        assert_eq!(result.unit_cost_in_cents, Some(5000));
        assert_eq!(result.invoice_number, Some("INV-001".to_string()));
        assert_eq!(result.notes, Some("Initial stock".to_string()));
        assert_eq!(result.place_id, None);
    }

    #[tokio::test]
    async fn should_create_stock_exit() {
        let _lock = lock_test_db().await;
        let pool = create_test_pool().await;
        clean_db(pool).await;

        let user = create_user(pool, None).await;
        let workspace = create_workspace(pool, user.id, Some("Test workspace".to_string())).await;
        let product = create_product(pool, workspace.id, "Keyboard").await;
        let place = create_place(pool, workspace.id, "Main Office").await;

        let repo = StockMovementsRepository::new(pool.clone());

        let movement_date = Utc::now();
        let result = repo
            .create_stock_exit(StockMovementExit {
                movement_date,
                product_id: product.id,
                place_id: place.id,
                quantity: -2,
                notes: Some("Employee delivery".to_string()),
            })
            .await
            .unwrap();

        assert!(result.id > 0);
        assert_eq!(result.product_id, product.id);
        assert_eq!(result.place_id, Some(place.id));
        assert_eq!(result.quantity, -2);
        assert_eq!(result.notes, Some("Employee delivery".to_string()));
        assert_eq!(result.supplier_id, None);
        assert_eq!(result.unit_cost_in_cents, None);
        assert_eq!(result.invoice_number, None);
    }

    #[tokio::test]
    async fn should_isolate_movements_between_workspaces() {
        let _lock = lock_test_db().await;
        let pool = create_test_pool().await;
        clean_db(pool).await;

        // Workspace A
        let user_a = create_user(pool, None).await;
        let ws_a = create_workspace(pool, user_a.id, Some("WS A".to_string())).await;
        let product_a = create_product(pool, ws_a.id, "Keyboard A").await;
        let supplier_a = create_supplier(pool, ws_a.id, "Supplier A").await;

        // Workspace B
        let user_b = create_user(pool, None).await;
        let ws_b = create_workspace(pool, user_b.id, Some("WS B".to_string())).await;
        let product_b = create_product(pool, ws_b.id, "Keyboard B").await;
        let supplier_b = create_supplier(pool, ws_b.id, "Supplier B").await;

        let repo = StockMovementsRepository::new(pool.clone());

        // Create entry for WS A
        let entry_a = repo
            .create_stock_entry(StockMovementEntry {
                movement_date: Utc::now(),
                product_id: product_a.id,
                supplier_id: supplier_a.id,
                quantity: 5,
                unit_cost_in_cents: 100,
                invoice_number: "INV-A".to_string(),
                notes: None,
            })
            .await
            .unwrap();

        // Create entry for WS B
        let entry_b = repo
            .create_stock_entry(StockMovementEntry {
                movement_date: Utc::now(),
                product_id: product_b.id,
                supplier_id: supplier_b.id,
                quantity: 10,
                unit_cost_in_cents: 200,
                invoice_number: "INV-B".to_string(),
                notes: None,
            })
            .await
            .unwrap();

        assert_ne!(entry_a.id, entry_b.id);
        assert_eq!(entry_a.product_id, product_a.id);
        assert_eq!(entry_b.product_id, product_b.id);

        // Ensure they have different suppliers
        assert_eq!(entry_a.supplier_id, Some(supplier_a.id));
        assert_eq!(entry_b.supplier_id, Some(supplier_b.id));
    }
}
