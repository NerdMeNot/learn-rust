package main

import (
	"cmp"
	"fmt"
)

func largest[T cmp.Ordered](list []T) T {
	best := list[0]
	for _, item := range list {
		if item > best {
			best = item
		}
	}
	return best
}

func main() {
	numbers := []int{3, 9, 4}
	words := []string{"pear", "fig", "apple"}
	fmt.Println("largest number:", largest(numbers))
	fmt.Println("largest word:", largest(words))
}
