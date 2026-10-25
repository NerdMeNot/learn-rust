package main

import "fmt"

func main() {
	// range over a string: the index is a byte offset, the second value a rune
	for i, c := range "aé!" {
		fmt.Printf("%d: %c\n", i, c)
	}
}
