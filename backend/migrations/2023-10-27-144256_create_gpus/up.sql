-- Your SQL goes here
CREATE TABLE gpus (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    model VARCHAR(100) NOT NULL,
    base_frequency INTEGER NOT NULL, -- in MHz
    boost_frequency INTEGER NOT NULL, -- also MHz
    vendor VARCHAR(100) NOT NULL,
    vendor_model VARCHAR(100) NOT NULL,
    memory INTEGER NOT NULL, -- IN MB
    memory_type VARCHAR(100) NOT NULL, -- i.e: GDDR6
    additional_features VARCHAR(100) NULL
)