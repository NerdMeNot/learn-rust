package main

import "fmt"

type Point struct{ X, Y int }

func (p Point) String() string { return fmt.Sprintf("(%d,%d)", p.X, p.Y) }

type Plain struct{ X, Y int }

type Wrap struct {
	P  Point
	ID int
}

func main() {
	p := Point{1, 2}
	fmt.Printf("%v | %+v | %s | %#v\n", p, p, p, p)
	fmt.Printf("%v | %+v\n", Plain{1, 2}, Plain{1, 2})
	fmt.Printf("%+v\n", Wrap{p, 3})
	fmt.Printf("%e %x\n", 1234.5, -255)
}
