-- Your SQL goes here
CREATE TABLE fans (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    vendor VARCHAR(100) NOT NULL,
    vendor_model VARCHAR(100) NOT NULL,
    rpm INTEGER NOT NULL,
    dimension INTEGER NOT NULL, -- in mm
    max_noise INTEGER NOT NULL -- in dB rounded to 1 (12.4 -> 12; 12.5 -> 13)
);