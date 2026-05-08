-- Your SQL goes here
create table IF not exists stock_movements (
    id SERIAL PRIMARY KEY,
    movement_date TIMESTAMP NOT NULL DEFAULT NOW(),
    product_id uuid NOT NULL REFERENCES products(id) ON DELETE RESTRICT,
    supplier_id INTEGER REFERENCES suppliers(id) ON DELETE SET NULL,
    place_id INTEGER REFERENCES places(id) ON DELETE SET NULL,
    quantity INTEGER NOT NULL,
    unit_cost_in_cents INTEGER,
    invoice_number TEXT,
    notes TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMP
)