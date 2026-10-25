package main

import (
	"fmt"
	"strings"
)

// Point is a point on a grid.
type Point struct{ x, y int }

func main() {
	p := Point{3, 4} // a line comment
	items := []int{1, 2, 3}
	if len(items) != 3 {
		panic("items should have 3 elements")
	}
	fmt.Printf("point: Point { x: %d, y: %d }\n", p.x, p.y)
	fmt.Printf("items: %s\n", strings.ReplaceAll(fmt.Sprint(items), " ", ", "))
	fmt.Printf("row: [%5d] [%-5d] [%05d]\n", 42, 42, 42)
	pi := fmt.Sprintf("%.2f", 3.14159)
	fmt.Printf("pi: %s {done}\n", pi)
}
