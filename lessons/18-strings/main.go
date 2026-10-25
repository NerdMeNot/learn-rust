package main

import (
	"fmt"
	"strings"
	"unicode/utf8"
)

func main() {
	owned := "héllo"
	view := owned[0:1]

	var b strings.Builder
	b.WriteString("go")
	b.WriteString(" + rust")
	s := b.String()

	fmt.Println(owned, view, s)
	fmt.Printf("bytes=%d chars=%d\n", len(owned), utf8.RuneCountInString(owned))
	for _, c := range owned {
		fmt.Printf("[%c]", c)
	}
	fmt.Println()
}
