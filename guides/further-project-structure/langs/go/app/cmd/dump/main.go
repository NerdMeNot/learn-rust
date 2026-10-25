package main

import (
	"fmt"
	"os"
	"strings"

	"example.com/wordstat"
)

func main() {
	counts, err := wordstat.Count(strings.Join(os.Args[1:], " "))
	if err != nil {
		fmt.Fprintln(os.Stderr, "dump:", err)
		os.Exit(1)
	}
	fmt.Println(wordstat.ToJSON(counts))
}
