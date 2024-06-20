-- Your SQL goes here
CREATE TABLE matches (
    id INT AUTO_INCREMENT NOT NULL PRIMARY KEY,
    winner INT NOT NULL,
    loser INT NOT NULL,
    FOREIGN KEY (winner) REFERENCES players(id),
    FOREIGN KEY (loser) REFERENCES players(id)
);