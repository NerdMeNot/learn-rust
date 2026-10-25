package main

import (
	"database/sql"
	"fmt"
	"log"

	_ "modernc.org/sqlite"
)

type Note struct {
	ID   int64
	Text string
	Done bool
}

func main() {
	log.SetFlags(0) // no timestamp in error lines
	db, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		log.Fatal(err)
	}
	defer db.Close()
	db.SetMaxOpenConns(1) // one connection, so every statement sees the same in-memory database

	_, err = db.Exec(`CREATE TABLE notes (
		id INTEGER PRIMARY KEY,
		text TEXT NOT NULL,
		done BOOLEAN NOT NULL DEFAULT 0
	)`)
	if err != nil {
		log.Fatal(err)
	}
	for _, text := range []string{"learn ownership", "write an axum service"} {
		if _, err := db.Exec("INSERT INTO notes (text) VALUES (?)", text); err != nil {
			log.Fatal(err)
		}
	}
	if _, err := db.Exec("UPDATE notes SET done = 1 WHERE id = ?", 1); err != nil {
		log.Fatal(err)
	}

	rows, err := db.Query("SELECT id, text, done FROM notes ORDER BY id")
	if err != nil {
		log.Fatal(err)
	}
	defer rows.Close()
	for rows.Next() {
		var n Note
		if err := rows.Scan(&n.ID, &n.Text, &n.Done); err != nil {
			log.Fatal(err)
		}
		mark := " "
		if n.Done {
			mark = "x"
		}
		fmt.Printf("[%s] %d %s\n", mark, n.ID, n.Text)
	}
	if err := rows.Err(); err != nil {
		log.Fatal(err)
	}

	var open int64
	if err := db.QueryRow("SELECT COUNT(*) FROM notes WHERE done = 0").Scan(&open); err != nil {
		log.Fatal(err)
	}
	fmt.Printf("%d still open\n", open)
}
