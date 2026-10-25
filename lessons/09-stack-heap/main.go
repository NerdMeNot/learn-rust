package main

import "fmt"

type Point struct {
	X, Y int
}

func main() {
	a := Point{1, 2}
	b := a // a struct is a value: b is a copy
	b.X = 99
	fmt.Printf("a = (%d, %d)\n", a.X, a.Y)
	fmt.Printf("b = (%d, %d)\n", b.X, b.Y)

	// To share, you take an address on purpose.
	p := &a
	p.X = 50
	fmt.Printf("a after p.X = 50: (%d, %d)\n", a.X, a.Y)
}
