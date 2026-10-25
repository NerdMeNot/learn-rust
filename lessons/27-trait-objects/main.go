package main

import "fmt"

type Shape interface {
	Name() string
	Area() int
}

type Square struct{ Side int }
type Rect struct{ W, H int }

func (Square) Name() string { return "square" }
func (s Square) Area() int  { return s.Side * s.Side }
func (Rect) Name() string   { return "rect" }
func (r Rect) Area() int    { return r.W * r.H }

func main() {
	shapes := []Shape{Square{3}, Rect{4, 6}}

	total := 0
	for _, s := range shapes {
		fmt.Printf("%s: %d\n", s.Name(), s.Area())
		total += s.Area()
	}
	fmt.Printf("total: %d\n", total)
}
