-- Your SQL goes here
CREATE TABLE rams (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    vendor VARCHAR(100) NOT NULL,
    vendor_model VARCHAR(100) NOT NULL,
    size INTEGER NOT NULL, -- in MB
    frequency INTEGER NOT NULL, -- in MHz
    cl INTEGER NOT NULL,
    mem_type VARCHAR(100) NOT NULL, -- i. e: DDR5
    form_factor VARCHAR(100) NOT NULL
);