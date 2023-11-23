-- Your SQL goes here
CREATE TABLE computer_rams(
    ram_id INTEGER NOT NULL,
    computer_id INTEGER NOT NULL,
    FOREIGN KEY(ram_id) REFERENCES rams(id),
    FOREIGN KEY(computer_id) REFERENCES computers(id),
    PRIMARY KEY(ram_id, computer_id)
)