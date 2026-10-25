package main

import "fmt"

type Item struct{ count int }

func main() {
	// A pointer into a slice, then an append that has to move the data.
	s := make([]int, 1, 1)
	p := &s[0]
	s = append(s, 2)
	*p = 9
	fmt.Println("s[0]:", s[0], " *p:", *p)

	// `range` hands you a copy of each element.
	items := []Item{{1}, {2}}
	for _, it := range items {
		it.count++
	}
	fmt.Println("after range:", items)
	for i := range items {
		items[i].count++
	}
	fmt.Println("after index:", items)
}
