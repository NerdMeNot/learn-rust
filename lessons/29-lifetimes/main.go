package main

import (
	"fmt"
	"strings"
)

// Nothing to annotate: the garbage collector keeps the text alive while anyone uses it.
func longest(a, b string) string {
	if len(a) >= len(b) {
		return a
	}
	return b
}

type Excerpt struct {
	part string
}

func main() {
	novel := "Call me Ishmael. Some years ago..."
	first, _, _ := strings.Cut(novel, ".")
	excerpt := Excerpt{part: first}
	l := longest(excerpt.part, "short")
	fmt.Println("longest:", l)
	fmt.Println("excerpt length:", len(excerpt.part))
}
