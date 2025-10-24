-- Add migration script here

CREATE TABLE shares (
    list_id SERIAL PRIMARY KEY,
    user_id SERIAL NOT NULL
);
