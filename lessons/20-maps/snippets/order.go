package main

import (
	"fmt"
	"maps"
	"slices"
)

func main() {
	m := map[string]int{}
	for i, k := range []string{"a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l"} {
		m[k] = i
	}

	// The same map, looped over three times in one run.
	for round := 1; round <= 3; round++ {
		fmt.Printf("loop %d:", round)
		for k := range m {
			fmt.Printf(" %s", k)
		}
		fmt.Println()
	}

	fmt.Println("sorted:", slices.Sorted(maps.Keys(m)))
}
