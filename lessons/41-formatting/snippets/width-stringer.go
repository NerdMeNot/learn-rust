package main

import "fmt"

type ID int

func (i ID) String() string { return fmt.Sprintf("#%d", int(i)) }

type Point struct{ X, Y int }

func main() {
	fmt.Printf("[%8v] [%-8v]\n", ID(42), ID(42)) // width applies to String()'s result
	fmt.Printf("%v | %+v | %#v | %T\n", Point{1, 2}, Point{1, 2}, Point{1, 2}, Point{1, 2})
	fmt.Printf("%d\n", "text")
	fmt.Printf("%d %d\n", 1)
	fmt.Printf("%.0f %.0f %.0f\n", 0.5, 1.5, 2.5)
}
