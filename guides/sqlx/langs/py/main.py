import sqlite3

db = sqlite3.connect(":memory:")
db.execute(
    """CREATE TABLE notes (
        id INTEGER PRIMARY KEY,
        text TEXT NOT NULL,
        done BOOLEAN NOT NULL DEFAULT 0
    )"""
)
for text in ["learn ownership", "write an axum service"]:
    db.execute("INSERT INTO notes (text) VALUES (?)", (text,))
db.execute("UPDATE notes SET done = 1 WHERE id = ?", (1,))

db.row_factory = sqlite3.Row
for row in db.execute("SELECT id, text, done FROM notes ORDER BY id"):
    mark = "x" if row["done"] else " "
    print(f"[{mark}] {row['id']} {row['text']}")

(open_count,) = db.execute("SELECT COUNT(*) FROM notes WHERE done = 0").fetchone()
print(f"{open_count} still open")
