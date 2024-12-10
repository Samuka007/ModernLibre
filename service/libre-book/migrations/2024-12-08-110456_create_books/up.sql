-- Your SQL goes here
CREATE TABLE books (
    id SERIAL PRIMARY KEY,
    cover_url VARCHAR,
    title VARCHAR,
    author VARCHAR,
    description TEXT,
    rating FLOAT8,
    added_date DATE NOT NULL,
    extension VARCHAR,
);
