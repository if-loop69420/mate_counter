-- Add migration script here
CREATE TABLE IF NOT EXISTS user (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT UNIQUE,
    password TEXT
);

CREATE TABLE IF NOT EXISTS friendship_statuses (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    description TEXT UNIQUE
);

INSERT INTO friendship_statuses (description) VALUES
    ('NOT BEFRIENDED'),
    ('FRIENDS'),
    ('PENDING'),
    ('REJECTED');

CREATE TABLE IF NOT EXISTS friendship (
    id_1 INTEGER,
    id_2 INTEGER,
    status_id INTEGER DEFAULT 0,
    mate_count INTEGER DEFAULT 0, -- Positive if id_1 is owed. Negative if id_2 is owed
    PRIMARY KEY (id_1, id_2),
    FOREIGN KEY(id_1) REFERENCES user(id),
    FOREIGN KEY(id_2) REFERENCES user(id),
    FOREIGN KEY(status_id) REFERENCES friendship_statuses(id),
    CHECK(id_1 < id_2)
);
