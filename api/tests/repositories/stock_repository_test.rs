#[cfg(test)]
mod tests {
    use chrono::Utc;
    use stock_management_api::{
        db::stock_movements_repository::StockMovementsRepository,
        db::stock_repository::ProductStockRepository,
        models::stock_movement::{StockMovementEntry, StockMovementExit},
    };

    use crate::common::{
        db::{clean_db, create_test_pool, lock_test_db},
        helpers::{create_place, create_product, create_supplier, create_user, create_workspace},
    };

    #[tokio::test]
    async fn should_calculate_stock_correctly() {
        let _lock = lock_test_db().await;
        let pool = create_test_pool().await;
        clean_db(pool).await;

        let user = create_user(pool, None).await;
        let workspace = create_workspace(pool, user.id, Some("Test workspace".to_string())).await;
        let product = create_product(pool, workspace.id, "Keyboard").await;
        let supplier = create_supplier(pool, workspace.id, "Logitech").await;
        let place = create_place(pool, workspace.id, "Main Office").await;

        let movements_repo = StockMovementsRepository::new(pool.clone());
        let stock_repo = ProductStockRepository::new(pool.clone());

        // Add 100 items
        movements_repo
            .create_stock_entry(StockMovementEntry {
                movement_date: Utc::now(),
                product_id: product.id,
                supplier_id: supplier.id,
                quantity: 100,
                unit_cost_in_cents: 5000,
                invoice_number: "INV-001".to_string(),
                notes: Some("Initial stock".to_string()),
            })
            .await
            .unwrap();

        // Remove 20 items
        movements_repo
            .create_stock_exit(StockMovementExit {
                movement_date: Utc::now(),
                product_id: product.id,
                place_id: place.id,
                quantity: -20,
                notes: Some("Employee delivery".to_string()),
            })
            .await
            .unwrap();

        // List products and check stock
        let result = stock_repo
            .list_products_with_stock(workspace.id, "")
            .await
            .unwrap();

        assert_eq!(result.len(), 1);
        let (p, stock) = &result[0];
        assert_eq!(p.id, product.id);
        assert_eq!(stock.current, 80);
        assert_eq!(stock.min, product.min_stock as i64);
    }

    #[tokio::test]
    async fn should_get_stock_by_product_id() {
        let _lock = lock_test_db().await;
        let pool = create_test_pool().await;
        clean_db(pool).await;

        let user = create_user(pool, None).await;
        let workspace = create_workspace(pool, user.id, Some("Test workspace".to_string())).await;
        let product = create_product(pool, workspace.id, "Keyboard").await;
        let supplier = create_supplier(pool, workspace.id, "Logitech").await;

        let movements_repo = StockMovementsRepository::new(pool.clone());
        let stock_repo = ProductStockRepository::new(pool.clone());

        movements_repo
            .create_stock_entry(StockMovementEntry {
                movement_date: Utc::now(),
                product_id: product.id,
                supplier_id: supplier.id,
                quantity: 50,
                unit_cost_in_cents: 5000,
                invoice_number: "INV-001".to_string(),
                notes: None,
            })
            .await
            .unwrap();

        let stock = stock_repo
            .get_stock_by_product_id(product.id)
            .await
            .unwrap();
        assert_eq!(stock, 50);
    }

    #[tokio::test]
    async fn should_isolate_stock_between_workspaces() {
        let _lock = lock_test_db().await;
        let pool = create_test_pool().await;
        clean_db(pool).await;

        // Workspace A
        let user_a = create_user(pool, None).await;
        let ws_a = create_workspace(pool, user_a.id, Some("WS A".to_string())).await;
        let product_a = create_product(pool, ws_a.id, "Product A").await;
        let supplier_a = create_supplier(pool, ws_a.id, "S A").await;

        // Workspace B
        let user_b = create_user(pool, None).await;
        let ws_b = create_workspace(pool, user_b.id, Some("WS B".to_string())).await;
        let product_b = create_product(pool, ws_b.id, "Product B").await;
        let supplier_b = create_supplier(pool, ws_b.id, "S B").await;

        let movements_repo = StockMovementsRepository::new(pool.clone());
        let stock_repo = ProductStockRepository::new(pool.clone());

        // Add stock to A
        movements_repo
            .create_stock_entry(StockMovementEntry {
                movement_date: Utc::now(),
                product_id: product_a.id,
                supplier_id: supplier_a.id,
                quantity: 10,
                unit_cost_in_cents: 100,
                invoice_number: "INV-A".to_string(),
                notes: None,
            })
            .await
            .unwrap();

        // Add stock to B
        movements_repo
            .create_stock_entry(StockMovementEntry {
                movement_date: Utc::now(),
                product_id: product_b.id,
                supplier_id: supplier_b.id,
                quantity: 20,
                unit_cost_in_cents: 200,
                invoice_number: "INV-B".to_string(),
                notes: None,
            })
            .await
            .unwrap();

        // Check A
        let result_a = stock_repo
            .list_products_with_stock(ws_a.id, "")
            .await
            .unwrap();
        assert_eq!(result_a.len(), 1);
        assert_eq!(result_a[0].1.current, 10);

        // Check B
        let result_b = stock_repo
            .list_products_with_stock(ws_b.id, "")
            .await
            .unwrap();
        assert_eq!(result_b.len(), 1);
        assert_eq!(result_b[0].1.current, 20);
    }
}
