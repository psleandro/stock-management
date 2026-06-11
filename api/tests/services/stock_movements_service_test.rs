#[cfg(test)]
mod tests {
    use chrono::Utc;
    use stock_management_api::{
        db::places_repository::PlacesRepository,
        db::products_repository::ProductsRepository,
        db::stock_movements_repository::StockMovementsRepository,
        db::stock_repository::ProductStockRepository,
        db::suppliers_repository::SuppliersRepository,
        errors::{ApplicationError, DomainError},
        models::dto::stock_movement_dto::{StockMovementEntryDto, StockMovementExitDto},
        services::stock_movements_service::StockMovementsService,
    };

    use crate::common::{
        db::{clean_db, create_test_pool, lock_test_db},
        helpers::{create_place, create_product, create_supplier, create_user, create_workspace},
    };

    fn setup_service(
        pool: deadpool_diesel::Pool<deadpool_diesel::Manager<diesel::PgConnection>>,
    ) -> StockMovementsService {
        StockMovementsService::new(
            ProductsRepository::new(pool.clone()),
            StockMovementsRepository::new(pool.clone()),
            ProductStockRepository::new(pool.clone()),
            SuppliersRepository::new(pool.clone()),
            PlacesRepository::new(pool.clone()),
        )
    }

    #[tokio::test]
    async fn should_respect_multitenancy_for_stock_entry() {
        let _lock = lock_test_db().await;
        let pool = create_test_pool().await;
        clean_db(pool).await;

        let user_a = create_user(pool, None).await;
        let ws_a = create_workspace(pool, user_a.id, Some("WS A".to_string())).await;

        let user_b = create_user(pool, None).await;
        let ws_b = create_workspace(pool, user_b.id, Some("WS B".to_string())).await;

        let product_b = create_product(pool, ws_b.id, "Keyboard B").await;
        let supplier_b = create_supplier(pool, ws_b.id, "Supplier B").await;

        let service = setup_service(pool.clone());

        // Try to create an entry in WS A using product and supplier from WS B
        let result = service
            .create_stock_entry(
                ws_a.id,
                StockMovementEntryDto {
                    movement_date: Utc::now(),
                    product_id: product_b.id,
                    supplier_id: supplier_b.id,
                    quantity: 10,
                    unit_cost_in_cents: 5000,
                    invoice_number: "INV-001".to_string(),
                    notes: None,
                },
            )
            .await;

        match result {
            Err(ApplicationError::DomainError(DomainError::ProductNotFound)) => {}
            _ => panic!(
                "Service should not allow access to products from another workspace; expected DomainError::ProductNotFound",
            ),
        }

        let product_a = create_product(pool, ws_a.id, "Keyboard A").await;

        // Try again with correct product but wrong supplier
        let result2 = service
            .create_stock_entry(
                ws_a.id,
                StockMovementEntryDto {
                    movement_date: Utc::now(),
                    product_id: product_a.id,
                    supplier_id: supplier_b.id,
                    quantity: 10,
                    unit_cost_in_cents: 5000,
                    invoice_number: "INV-001".to_string(),
                    notes: None,
                },
            )
            .await;

        match result2 {
            Err(ApplicationError::DomainError(DomainError::SupplierNotFound)) => {}
            _ => panic!(
                "Service should not allow access to suppliers from another workspace; expected DomainError::SupplierNotFound"
            ),
        }
    }

    #[tokio::test]
    async fn should_respect_multitenancy_for_stock_exit() {
        let _lock = lock_test_db().await;
        let pool = create_test_pool().await;
        clean_db(pool).await;

        let user_a = create_user(pool, None).await;
        let ws_a = create_workspace(pool, user_a.id, Some("WS A".to_string())).await;

        let user_b = create_user(pool, None).await;
        let ws_b = create_workspace(pool, user_b.id, Some("WS B".to_string())).await;

        let product_b = create_product(pool, ws_b.id, "Keyboard B").await;
        let place_b = create_place(pool, ws_b.id, "Place B").await;

        let service = setup_service(pool.clone());

        // Try to create an exit in WS A using product and place from WS B
        let result = service
            .create_stock_exit(
                ws_a.id,
                StockMovementExitDto {
                    movement_date: Utc::now(),
                    product_id: product_b.id,
                    place_id: place_b.id,
                    quantity: 5,
                    notes: None,
                },
            )
            .await;

        match result {
            Err(ApplicationError::DomainError(DomainError::ProductNotFound)) => {}
            _ => panic!(
                "Service should not allow access to products from another workspace; expected DomainError::ProductNotFound",
            ),
        }

        let product_a = create_product(pool, ws_a.id, "Keyboard A").await;

        // Try again with correct product but wrong place
        let result2 = service
            .create_stock_exit(
                ws_a.id,
                StockMovementExitDto {
                    movement_date: Utc::now(),
                    product_id: product_a.id,
                    place_id: place_b.id,
                    quantity: 5,
                    notes: None,
                },
            )
            .await;

        match result2 {
            Err(ApplicationError::DomainError(DomainError::PlaceNotFound)) => {}
            _ => panic!(
                "Service should not allow access to places from another workspace; expected DomainError::PlaceNotFound"
            ),
        }
    }

    #[tokio::test]
    async fn should_not_allow_stock_exit_when_not_enough_stock() {
        let _lock = lock_test_db().await;
        let pool = create_test_pool().await;
        clean_db(pool).await;

        let user = create_user(pool, None).await;
        let ws = create_workspace(pool, user.id, Some("WS".to_string())).await;
        let product = create_product(pool, ws.id, "Keyboard").await;
        let place = create_place(pool, ws.id, "Main Office").await;
        let supplier = create_supplier(pool, ws.id, "Supplier").await;

        let service = setup_service(pool.clone());

        service
            .create_stock_entry(
                ws.id,
                StockMovementEntryDto {
                    movement_date: Utc::now(),
                    product_id: product.id,
                    supplier_id: supplier.id,
                    quantity: 10,
                    unit_cost_in_cents: 5000,
                    invoice_number: "INV-001".to_string(),
                    notes: None,
                },
            )
            .await
            .expect("Should create stock entry successfully");

        let result = service
            .create_stock_exit(
                ws.id,
                StockMovementExitDto {
                    movement_date: Utc::now(),
                    product_id: product.id,
                    place_id: place.id,
                    quantity: 15,
                    notes: None,
                },
            )
            .await;

        match result {
            Err(ApplicationError::DomainError(DomainError::NotEnoughStock)) => {}
            _ => panic!(
                "Service should not allow removing more items than available in stock; expected DomainError::NotEnoughStock"
            ),
        }
    }

    #[tokio::test]
    async fn should_not_allow_stock_entry_with_deleted_product() {
        let _lock = lock_test_db().await;
        let pool = create_test_pool().await;
        clean_db(&pool).await;

        let user = create_user(&pool, None).await;
        let ws = create_workspace(&pool, user.id, None).await;
        let product = create_product(&pool, ws.id, "Keyboard").await;
        let supplier = create_supplier(&pool, ws.id, "Supplier").await;

        let products_repo = ProductsRepository::new(pool.clone());
        products_repo
            .delete_product(ws.id, product.id)
            .await
            .unwrap();

        let service = setup_service(pool.clone());

        let result = service
            .create_stock_entry(
                ws.id,
                StockMovementEntryDto {
                    movement_date: Utc::now(),
                    product_id: product.id,
                    supplier_id: supplier.id,
                    quantity: 10,
                    unit_cost_in_cents: 5000,
                    invoice_number: "INV-001".to_string(),
                    notes: None,
                },
            )
            .await;

        match result {
            Err(ApplicationError::DomainError(DomainError::ProductNotFound)) => {}
            _ => panic!(
                "Service should not allow using a deleted product; expected DomainError::ProductNotFound"
            ),
        }
    }

    #[tokio::test]
    async fn should_not_allow_stock_entry_with_deleted_supplier() {
        let _lock = lock_test_db().await;
        let pool = create_test_pool().await;
        clean_db(&pool).await;

        let user = create_user(&pool, None).await;
        let ws = create_workspace(&pool, user.id, None).await;
        let product = create_product(&pool, ws.id, "Keyboard").await;
        let supplier = create_supplier(&pool, ws.id, "Supplier").await;

        let suppliers_repo = SuppliersRepository::new(pool.clone());
        suppliers_repo
            .delete_supplier(ws.id, supplier.id)
            .await
            .unwrap();

        let service = setup_service(pool.clone());

        let result = service
            .create_stock_entry(
                ws.id,
                StockMovementEntryDto {
                    movement_date: Utc::now(),
                    product_id: product.id,
                    supplier_id: supplier.id,
                    quantity: 10,
                    unit_cost_in_cents: 5000,
                    invoice_number: "INV-001".to_string(),
                    notes: None,
                },
            )
            .await;

        match result {
            Err(ApplicationError::DomainError(DomainError::SupplierNotFound)) => {}
            _ => panic!(
                "Service should not allow using a deleted supplier; expected DomainError::SupplierNotFound"
            ),
        }
    }

    #[tokio::test]
    async fn should_not_allow_stock_exit_with_deleted_product() {
        let _lock = lock_test_db().await;
        let pool = create_test_pool().await;
        clean_db(&pool).await;

        let user = create_user(&pool, None).await;
        let ws = create_workspace(&pool, user.id, None).await;
        let product = create_product(&pool, ws.id, "Keyboard").await;
        let place = create_place(&pool, ws.id, "Main Office").await;
        let supplier = create_supplier(&pool, ws.id, "Supplier").await;

        let service = setup_service(pool.clone());

        // Add some stock first
        service
            .create_stock_entry(
                ws.id,
                StockMovementEntryDto {
                    movement_date: Utc::now(),
                    product_id: product.id,
                    supplier_id: supplier.id,
                    quantity: 10,
                    unit_cost_in_cents: 5000,
                    invoice_number: "INV-001".to_string(),
                    notes: None,
                },
            )
            .await
            .unwrap();

        let products_repo = ProductsRepository::new(pool.clone());
        products_repo
            .delete_product(ws.id, product.id)
            .await
            .unwrap();

        let result = service
            .create_stock_exit(
                ws.id,
                StockMovementExitDto {
                    movement_date: Utc::now(),
                    product_id: product.id,
                    place_id: place.id,
                    quantity: 5,
                    notes: None,
                },
            )
            .await;

        match result {
            Err(ApplicationError::DomainError(DomainError::ProductNotFound)) => {}
            _ => panic!(
                "Service should not allow using a deleted product in stock exit; expected DomainError::ProductNotFound"
            ),
        }
    }

    #[tokio::test]
    async fn should_not_allow_stock_exit_with_deleted_place() {
        let _lock = lock_test_db().await;
        let pool = create_test_pool().await;
        clean_db(&pool).await;

        let user = create_user(&pool, None).await;
        let ws = create_workspace(&pool, user.id, None).await;
        let product = create_product(&pool, ws.id, "Keyboard").await;
        let place = create_place(&pool, ws.id, "Main Office").await;
        let supplier = create_supplier(&pool, ws.id, "Supplier").await;

        let service = setup_service(pool.clone());

        // Add some stock first
        service
            .create_stock_entry(
                ws.id,
                StockMovementEntryDto {
                    movement_date: Utc::now(),
                    product_id: product.id,
                    supplier_id: supplier.id,
                    quantity: 10,
                    unit_cost_in_cents: 5000,
                    invoice_number: "INV-001".to_string(),
                    notes: None,
                },
            )
            .await
            .unwrap();

        let places_repo = PlacesRepository::new(pool.clone());
        places_repo.delete_place(ws.id, place.id).await.unwrap();

        let result = service
            .create_stock_exit(
                ws.id,
                StockMovementExitDto {
                    movement_date: Utc::now(),
                    product_id: product.id,
                    place_id: place.id,
                    quantity: 5,
                    notes: None,
                },
            )
            .await;

        match result {
            Err(ApplicationError::DomainError(DomainError::PlaceNotFound)) => {}
            _ => panic!(
                "Service should not allow using a deleted place; expected DomainError::PlaceNotFound"
            ),
        }
    }
}
