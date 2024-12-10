-- Create tables if they do not exist
CREATE TABLE IF NOT EXISTS books (
    id SERIAL PRIMARY KEY,  -- 书本 ID
    title VARCHAR(255) NOT NULL, -- 书本标题
    author VARCHAR(255) NOT NULL, -- 作者
    description TEXT,   -- 书本描述
    status INT, -- 0: Inactive, 1: Active
    rating FLOAT,   -- 评分
    added_date DATE,  -- 添加日期
    cover_url VARCHAR(255),  -- 书本封面的 URL
    extension VARCHAR(255)  -- 书本文件的扩展名
);

-- CREATE TABLE IF NOT EXISTS READINGLIST (
--     id SERIAL PRIMARY KEY,  -- 书单 ID
--     user_id INT REFERENCES "USER"(id),  -- 用户 ID
--     name VARCHAR(255) NOT NULL, -- 书单名称
--     is_public BOOLEAN
-- );

-- CREATE TABLE IF NOT EXISTS READINGLISTBOOK (
--     reading_list_id INT REFERENCES READINGLIST(id), -- 书单 ID
--     book_id INT REFERENCES BOOKS(id),    -- 书本 ID
--     PRIMARY KEY (reading_list_id, book_id)
-- );

-- CREATE TABLE IF NOT EXISTS SUBSCRIPTION (
--     user_id INT REFERENCES "USER"(id),  -- 用户 ID
--     reading_list_id INT REFERENCES READINGLIST(id), -- 书单 ID
--     PRIMARY KEY (user_id, reading_list_id)
-- );

-- CREATE TABLE IF NOT EXISTS chapters (
--     id SERIAL PRIMARY KEY,
--     title VARCHAR(255) NOT NULL,
--     index INT NOT NULL,
--     content TEXT NOT NULL,
--     level INT NOT NULL,
--     parent_id INT,
--     book_id INT REFERENCES BOOKS(id),
--     created_time DATE,
--     updated_time DATE
-- );

-- Insert test data
INSERT INTO books (title, author, description, status, rating, added_date, cover_url, extension) VALUES
('Book One', 'Author One', 'Description One', 1, 4.5, '2023-01-01', 'cover_url_1', 'pdf'),
('Book Two', 'Author Two', 'Description Two', 1, 4.0, '2023-02-01', 'cover_url_2', 'epub');

-- INSERT INTO READINGLIST (user_id, name, is_public) VALUES
-- (1, 'Reading List One', TRUE),
-- (2, 'Reading List Two', FALSE);

-- INSERT INTO READINGLISTBOOK (reading_list_id, book_id) VALUES
-- (1, 1),
-- (2, 2);

-- INSERT INTO SUBSCRIPTION (user_id, reading_list_id) VALUES
-- (1, 2),
-- (2, 1);