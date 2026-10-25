package main

import (
	"encoding/json"
	"fmt"
	"net/http"
	"strconv"
	"sync"
)

type Note struct {
	ID   int    `json:"id"`
	Text string `json:"text"`
}

type App struct {
	mu    sync.Mutex
	notes []Note
}

func (a *App) list(w http.ResponseWriter, r *http.Request) {
	a.mu.Lock()
	defer a.mu.Unlock()
	writeJSON(w, http.StatusOK, a.notes)
}

func (a *App) create(w http.ResponseWriter, r *http.Request) {
	var input struct {
		Text string `json:"text"`
	}
	if err := json.NewDecoder(r.Body).Decode(&input); err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	a.mu.Lock()
	defer a.mu.Unlock()
	note := Note{ID: len(a.notes) + 1, Text: input.Text}
	a.notes = append(a.notes, note)
	writeJSON(w, http.StatusCreated, note)
}

func (a *App) getOne(w http.ResponseWriter, r *http.Request) {
	id, _ := strconv.Atoi(r.PathValue("id"))
	a.mu.Lock()
	defer a.mu.Unlock()
	for _, n := range a.notes {
		if n.ID == id {
			writeJSON(w, http.StatusOK, n)
			return
		}
	}
	w.WriteHeader(http.StatusNotFound)
}

func writeJSON(w http.ResponseWriter, status int, v any) {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(status)
	json.NewEncoder(w).Encode(v)
}

func main() {
	app := &App{notes: []Note{}}
	mux := http.NewServeMux()
	mux.HandleFunc("GET /notes", app.list)
	mux.HandleFunc("POST /notes", app.create)
	mux.HandleFunc("GET /notes/{id}", app.getOne)

	fmt.Println("listening on http://127.0.0.1:3000")
	http.ListenAndServe("127.0.0.1:3000", mux)
}
