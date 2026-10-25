package main

import (
	"cmp"
	"fmt"
)

func Max[T cmp.Ordered](a, b T) T {
	if b > a {
		return b
	}
	return a
}

type Point struct{ x int }

func main() {
	fmt.Println(Max(1, 2))
	fmt.Println(Max(Point{1}, Point{2}))
}
