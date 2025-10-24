-- Add migration script here

CREATE TABLE lists (
    id SERIAL PRIMARY KEY,
    user_id SERIAL REFERENCES users(id) ON DELETE CASCADE,
    title TEXT NOT NULL
);
