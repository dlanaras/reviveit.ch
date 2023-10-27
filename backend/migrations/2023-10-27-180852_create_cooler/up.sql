-- Your SQL goes here
CREATE TABLE coolers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    vendor VARCHAR(100) NOT NULL,
    vendor_model VARCHAR(100) NOT NULL,
    cooler_type VARCHAR(100) NOT NULL, -- i. e: Water cooling (hybrid) | air | passive...
    fan_count INTEGER NOT NULL,
    fan_dimension INTEGER NOT NULL -- in mm
);