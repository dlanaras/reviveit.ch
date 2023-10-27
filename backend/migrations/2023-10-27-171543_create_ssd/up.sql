-- Your SQL goes here
CREATE TABLE ssds (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    vendor VARCHAR(100) NOT NULL,
    vendor_model VARCHAR(100) NOT NULL,
    storage INTEGER NOT NULL, -- in GB
    form_factor VARCHAR(100) NOT NULL,
    read_speed INTEGER NOT NULL, -- in MB
    write_speed INTEGER NOT NULL -- in MB
);