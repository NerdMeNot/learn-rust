package main

import (
	"fmt"
	"strings"
)

type Point struct{ x, y int }

func main() {
	// no destructuring of a struct: read the fields
	p := Point{3, -2}
	x, y := p.x, p.y
	fmt.Printf("x = %d, y = %d\n", x, y)

	// range gives (index, rune) pairs
	for i, c := range "abc" {
		fmt.Printf("%d: %c\n", i, c)
	}

	// strings.Cut returns a found flag
	if key, value, found := strings.Cut("mode=fast", "="); found {
		fmt.Printf("%s -> %s\n", key, value)
	}

	// an array is read by index
	a := [4]int{10, 20, 30, 40}
	first, last := a[0], a[len(a)-1]
	fmt.Printf("first = %d, last = %d\n", first, last)
}
