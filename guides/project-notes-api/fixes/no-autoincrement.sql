CREATE TABLE notes (
    id INTEGER PRIMARY KEY,
    text TEXT NOT NULL CHECK (length(text) > 0)
);
