-- Your SQL goes here
CREATE TABLE articles(
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title VARCHAR(100) NOT NULL,
    date INTEGER NOT NULL, -- sqlite does not have a date type so its gonna be saved as unix time
    content VARCHAR(1000)
)