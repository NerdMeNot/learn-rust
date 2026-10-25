package main

import "fmt"

type Shape interface{ Area() int }

type Square struct{ Side int }

func (s *Square) Area() int {
	if s == nil {
		return -1
	}
	return s.Side * s.Side
}

func find(want bool) Shape {
	var p *Square // a nil pointer
	if want {
		p = &Square{3}
	}
	return p // the interface now holds (type *Square, value nil)
}

func main() {
	s := find(false)
	fmt.Println("find(false) == nil:", s == nil)
	fmt.Println("area:", s.Area())
}
