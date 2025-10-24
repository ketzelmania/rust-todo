-- Add migration script here

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    session_key TEXT
);

INSERT INTO users (username, password) VALUES ('joe123', 'abc');
