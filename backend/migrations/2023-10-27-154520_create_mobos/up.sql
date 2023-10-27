-- Your SQL goes here
CREATE TABLE mobos (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    vendor VARCHAR(100) NOT NULL,
    vendor_model VARCHAR(100) NOT NULL,
    chipset VARCHAR(100) NOT NULL,
    socket VARCHAR(100) NOT NULL,
    memory_type VARCHAR(100) NOT NULL,
    max_memory INTEGER NOT NULL, -- in GB
    memory_slots INTEGER NOT NULL,
    form_factor VARCHAR(100) NOT NULL,
    additional_features VARCHAR(255) NULL
)