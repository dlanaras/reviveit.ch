-- Your SQL goes here
CREATE TABLE cases (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    vendor VARCHAR(100) NOT NULL,
    vendor_model VARCHAR(100) NOT NULL,
    form_factor VARCHAR(100) NOT NULL,
    fan_slots INTEGER NOT NULL,
    installed_fans INTEGER NOT NULL,
    hdd_slots INTEGER NOT NULL,
    ssd_slots INTEGER NOT NULL
);