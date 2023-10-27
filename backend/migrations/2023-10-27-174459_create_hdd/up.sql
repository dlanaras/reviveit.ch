-- Your SQL goes here
CREATE TABLE hdds (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    vendor VARCHAR(100) NOT NULL,
    vendor_model VARCHAR(100) NOT NULL,
    storage INTEGER NOT NULL, -- in GB
    form_factor VARCHAR(100) NOT NULL,
    rpm INTEGER NOT NULL
);