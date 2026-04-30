-- Your SQL goes here
CREATE INDEX idx_products_workspace_not_deleted
ON products (workspace_id)
WHERE deleted_at IS NULL;

CREATE INDEX idx_places_workspace_not_deleted
ON places (workspace_id)
WHERE deleted_at IS NULL;

CREATE INDEX idx_suppliers_workspace_not_deleted
ON suppliers (workspace_id)
WHERE deleted_at IS NULL;

CREATE INDEX idx_products_stock_movements_not_deleted
ON stock_movements (product_id)
WHERE deleted_at IS NULL;