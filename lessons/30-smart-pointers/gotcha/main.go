package main

import "fmt"

type Counter struct{ Hits int }

func main() {
	a := &Counter{}
	b := a

	a.Hits++
	b.Hits++
	fmt.Println("hits:", a.Hits)
}
