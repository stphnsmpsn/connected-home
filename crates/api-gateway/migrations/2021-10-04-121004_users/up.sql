-- Your SQL goes here
CREATE TABLE users (
    username Text PRIMARY KEY NOT NULL,
    password Text NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    locked BOOLEAN NOT NULL
)