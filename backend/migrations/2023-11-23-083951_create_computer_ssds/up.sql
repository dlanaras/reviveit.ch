-- Your SQL goes here
CREATE TABLE computer_ssds(
    ssd_id INTEGER NOT NULL,
    computer_id INTEGER NOT NULL,
    FOREIGN KEY(ssd_id) REFERENCES ssds(id),
    FOREIGN KEY(computer_id) REFERENCES computers(id),
    PRIMARY KEY(ssd_id, computer_id)
)