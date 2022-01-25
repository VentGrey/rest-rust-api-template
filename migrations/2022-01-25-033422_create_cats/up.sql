-- Your SQL goes here
CREATE TABLE IF NOT EXISTS cats (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    photo_url TEXT NOT NULL,
    is_adopted BOOLEAN NOT NULL,
    description TEXT NOT NULL
);
