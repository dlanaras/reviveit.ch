-- Your SQL goes here
CREATE TABLE cpus (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    model VARCHAR(100) NOT NULL,
    base_frequency INTEGER NOT NULL, -- in MHz
    boost_frequency INTEGER NOT NULL, -- also MHz
    cores INTEGER NOT NULL
);