import { DatabaseSync } from "node:sqlite";

type Note = { id: number; text: string; done: number };

const db = new DatabaseSync(":memory:");
db.exec(`CREATE TABLE notes (
  id INTEGER PRIMARY KEY,
  text TEXT NOT NULL,
  done BOOLEAN NOT NULL DEFAULT 0
)`);
const insert = db.prepare("INSERT INTO notes (text) VALUES (?)");
for (const text of ["learn ownership", "write an axum service"]) {
  insert.run(text);
}
db.prepare("UPDATE notes SET done = 1 WHERE id = ?").run(1);

const notes = db.prepare("SELECT id, text, done FROM notes ORDER BY id").all() as Note[];
for (const note of notes) {
  const mark = note.done ? "x" : " ";
  console.log(`[${mark}] ${note.id} ${note.text}`);
}

const row = db.prepare("SELECT COUNT(*) AS n FROM notes WHERE done = 0").get() as { n: number };
console.log(`${row.n} still open`);
