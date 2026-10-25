package main

import (
	"fmt"
	"slices"
)

func main() {
	a := []int{1, 2, 3, 4}
	b := a[:2]
	b = append(b, 99)
	fmt.Println("a:", a, "b:", b)

	// No spare capacity: append has to copy, and `d` is untouched.
	d := []int{1, 2}
	e := d[:2]
	e = append(e, 5)
	fmt.Println("d:", d, "e:", e)

	// Two ways to cut the sharing: a full slice expression, or a clone.
	a = []int{1, 2, 3, 4}
	f := a[:2:2]
	f = append(f, 99)
	g := slices.Clone(a[:2])
	g = append(g, 7)
	fmt.Println("a:", a, "f:", f, "g:", g)
}
