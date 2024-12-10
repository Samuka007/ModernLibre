-- Your SQL goes here
CREATE TABLE IF NOT EXISTS categories (
    id SERIAL PRIMARY KEY,  -- 分类 ID
    name VARCHAR(255) NOT NULL  -- 分类名称
);

-- CREATE TABLE IF NOT EXISTS bookcategories (
--     book_id INT REFERENCES BOOKS(id),    -- 书本 ID
--     category_id INT REFERENCES CATEGORY(id),    -- 分类 ID
--     PRIMARY KEY (book_id, category_id)
-- );

INSERT INTO categories (name) VALUES
('Fiction'),
('Non-Fiction');

-- INSERT INTO BOOKCATEGORY (book_id, category_id) VALUES
-- (1, 1),
-- (2, 2);
