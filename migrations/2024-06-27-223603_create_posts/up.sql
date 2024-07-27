-- Your SQL goes here
CREATE TABLE IF NOT EXISTS post (
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    slug VARCHAR NOT NULL,
    body TEXT NOT NULL
);