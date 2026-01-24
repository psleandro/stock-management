-- Your SQL goes here
create table IF not exists workspaces (
  id SERIAL PRIMARY KEY,
  name TEXT, 
  owner_id INTEGER NOT NULL REFERENCES users(id),
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
  deleted_at TIMESTAMP,

  UNIQUE(owner_id)
)