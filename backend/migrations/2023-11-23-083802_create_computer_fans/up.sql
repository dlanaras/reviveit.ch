-- Your SQL goes here
CREATE TABLE computer_fans(
    fan_id INTEGER NOT NULL,
    computer_id INTEGER NOT NULL,
    FOREIGN KEY(fan_id) REFERENCES fans(id),
    FOREIGN KEY(computer_id) REFERENCES computers(id),
    PRIMARY KEY(fan_id, computer_id)
)