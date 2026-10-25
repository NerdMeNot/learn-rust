package main

import (
	"cmp"
	"fmt"
	"runtime"
)

// Backs the Go column of the table in lesson.mdx. Never shown.

type Stack[T any] struct{ items []T }

func (s *Stack[T]) Push(x T) { s.items = append(s.items, x) }

func Index[T comparable](s []T, x T) int {
	for i, y := range s {
		if y == x {
			return i
		}
	}
	return -1
}

func Max[T cmp.Ordered](a, b T) T {
	if b > a {
		return b
	}
	return a
}

type Number interface{ ~int | ~float64 }

type MyInt int

func Sum[T Number](xs []T) T {
	var total T
	for _, x := range xs {
		total += x
	}
	return total
}

type Point struct{ X, Y int }

func (p Point) String() string { return fmt.Sprintf("(%d,%d)", p.X, p.Y) }

func (p Point) Less(other Point) bool { return p.X < other.X }

// A constraint can require a method: fmt.Stringer, or one of your own.
func Show[T fmt.Stringer](x T) string { return "<" + x.String() + ">" }

func Smaller[T interface{ Less(T) bool }](a, b T) T {
	if b.Less(a) {
		return b
	}
	return a
}

// Methods with their own type parameters need Go 1.27.
func (s Stack[T]) Map[U any](f func(T) U) []U {
	out := []U{}
	for _, x := range s.items {
		out = append(out, f(x))
	}
	return out
}

func Map[T, U any](s []T, f func(T) U) []U {
	out := []U{}
	for _, x := range s {
		out = append(out, f(x))
	}
	return out
}

func main() {
	fmt.Println("toolchain:", runtime.Version())
	fmt.Println("Show with fmt.Stringer:", Show(Point{1, 2}))
	fmt.Println("Smaller with a Less method:", Smaller(Point{3, 0}, Point{1, 9}))
	fmt.Println("Max of MyInt:", Max(MyInt(3), MyInt(5)))
	var s Stack[int]
	s.Push(4)
	fmt.Println("stack:", s.items)
	fmt.Println("method with its own type parameter:", s.Map(func(n int) string { return fmt.Sprint(n * 2) }))
	fmt.Println("Index:", Index([]string{"a", "b"}, "b"))
	fmt.Println("Max:", Max("kiwi", "pear"))
	fmt.Println("Sum of MyInt:", Sum([]MyInt{1, 2, 3}))
	fmt.Println("Map explicit:", Map[int, string]([]int{1, 2}, func(n int) string { return fmt.Sprint(n * 2) }))
}
