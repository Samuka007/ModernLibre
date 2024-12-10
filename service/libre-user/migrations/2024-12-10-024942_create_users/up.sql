-- Your SQL goes here
CREATE TABLE users (
    uid UUID PRIMARY KEY,
    login VARCHAR NOT NULL,
    name VARCHAR,
    avatar VARCHAR,
    email VARCHAR,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    admin BOOLEAN NOT NULL DEFAULT FALSE,
    github_id BIGINT,
    casdoor_id VARCHAR
);