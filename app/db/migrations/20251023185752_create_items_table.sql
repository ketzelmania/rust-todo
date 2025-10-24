-- Add migration script here

CREATE TABLE items (
    id SERIAL PRIMARY KEY,
    list_id SERIAL REFERENCES lists(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    status TEXT,
    description TEXT
);
