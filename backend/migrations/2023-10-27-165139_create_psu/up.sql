-- Your SQL goes here
CREATE TABLE psus (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    vendor VARCHAR(100) NOT NULL,
    vendor_model VARCHAR(100) NOT NULL,
    wattage INTEGER NOT NULL,
    eighty_plus VARCHAR(100) NOT NULL,
    form_factor VARCHAR(100) NOT NULL
);