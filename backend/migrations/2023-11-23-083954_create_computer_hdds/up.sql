-- Your SQL goes here
CREATE TABLE computer_hdds(
    hdd_id INTEGER NOT NULL,
    computer_id INTEGER NOT NULL,
    FOREIGN KEY(hdd_id) REFERENCES hdds(id),
    FOREIGN KEY(computer_id) REFERENCES computers(id),
    PRIMARY KEY(hdd_id, computer_id)
)