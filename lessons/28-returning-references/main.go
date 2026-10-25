package main

import (
	"fmt"
	"strings"
)

// a Go function would normally return the string itself; `&s` is the shape Rust rejects
func makeGreeting(name string) *string {
	s := "hello " + name
	return &s // fine in Go: s escapes, so the compiler allocates it on the heap
}

// returns a piece of the input: no copy
func firstWord(text string) string {
	word, _, _ := strings.Cut(text, " ")
	return word
}

func main() {
	g := makeGreeting("rust")
	w := firstWord(*g)
	fmt.Println(*g, "/", w)
}
