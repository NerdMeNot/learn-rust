package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"net/http"
)

type Note struct {
	ID   int    `json:"id"`
	Text string `json:"text"`
}

func main() {
	body, _ := json.Marshal(map[string]string{"text": "sent from Go"})
	resp, err := http.Post("http://127.0.0.1:3000/notes", "application/json", bytes.NewReader(body))
	if err != nil {
		panic(err)
	}
	defer resp.Body.Close()
	if resp.StatusCode >= 400 {
		panic(resp.Status)
	}
	var created Note
	json.NewDecoder(resp.Body).Decode(&created)
	fmt.Printf("created %+v\n", created)

	resp2, err := http.Get("http://127.0.0.1:3000/notes")
	if err != nil {
		panic(err)
	}
	defer resp2.Body.Close()
	var all []Note
	json.NewDecoder(resp2.Body).Decode(&all)
	fmt.Printf("the server now has %d notes\n", len(all))
}
