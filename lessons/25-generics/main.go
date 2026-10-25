package main

import (
	"cmp"
	"fmt"
)

func largest[T cmp.Ordered](items []T) T {
	best := items[0]
	for _, item := range items {
		if item > best {
			best = item
		}
	}
	return best
}

func main() {
	fmt.Println(largest([]int{3, 9, 2}))
	fmt.Println(largest([]string{"kiwi", "apple", "pear"}))
}
