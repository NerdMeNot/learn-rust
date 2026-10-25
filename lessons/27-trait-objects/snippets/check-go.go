package main

import "fmt"

type Shape interface{ Area() int }

type Named interface{ Name() string }

type Both interface {
	Shape
	Named
}

type Square struct{ Side int }
type Rect struct{ W, H int }

func (s Square) Area() int    { return s.Side * s.Side }
func (s Square) Name() string { return "square" }
func (r Rect) Area() int      { return r.W * r.H }

func f[T Shape](s T) int { return s.Area() }

func main() {
	// the type-parameter form
	if f(Square{2}) != 4 {
		panic("generic")
	}

	shapes := []Shape{Square{3}, Rect{4, 6}}

	// s.(Square): a type assertion with ok
	sq, ok := shapes[0].(Square)
	if !ok || sq.Side != 3 {
		panic("assertion")
	}
	if _, ok := shapes[1].(Square); ok {
		panic("a Rect is not a Square")
	}

	// switch v := s.(type)
	kinds := ""
	for _, s := range shapes {
		switch v := s.(type) {
		case Square:
			kinds += fmt.Sprint("square", v.Side, " ")
		case Rect:
			kinds += fmt.Sprint("rect", v.W, " ")
		}
	}
	if kinds != "square3 rect4 " {
		panic(kinds)
	}

	// the empty interface holds anything
	var anything any = 42
	if _, ok := anything.(int); !ok {
		panic("any")
	}

	// an interface that embeds two others
	var b Both = Square{2}
	if b.Area() != 4 || b.Name() != "square" {
		panic("embedding")
	}

	// a nil interface
	var none Shape
	if none != nil {
		panic("nil")
	}
	fmt.Println("ok")
}
