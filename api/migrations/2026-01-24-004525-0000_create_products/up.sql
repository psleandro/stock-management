-- Your SQL goes here
create type base_unit as ENUM (
    'unit',
    'milligram',
    'milliliter'
);

create table IF not exists products (
    id SERIAL PRIMARY KEY,
    workspace_id INTEGER NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    base_unit base_unit NOT NULL,
    brand TEXT,
    min_stock BIGINT NOT NULL DEFAULT 0,
    observation TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMP
)