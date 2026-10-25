package main

import "fmt"

type Shape interface{ Area() int }

type Square struct{ Side int }
type Rect struct{ W, H int }

func (s Square) Area() int { return s.Side * s.Side }
func (r Rect) Area() int   { return r.W * r.H }

func main() {
	shapes := []Shape{Square{3}, Rect{4, 6}}

	total := 0
	for _, s := range shapes {
		total += s.Area()
	}
	fmt.Println("total:", total)
}
