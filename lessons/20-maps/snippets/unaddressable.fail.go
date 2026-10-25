package main

import "fmt"

type Stats struct{ count int }

func main() {
	m := map[string]Stats{"a": {}}
	m["a"].count++
	fmt.Println(m)
}
