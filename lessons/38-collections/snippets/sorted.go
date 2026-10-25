package main

import (
	"fmt"
	"slices"
)

func main() {
	// Go's standard library has no sorted map: keep a sorted slice and search it.
	keys := []int{}
	for _, k := range []int{40, 10, 25} {
		i, found := slices.BinarySearch(keys, k)
		if !found {
			keys = slices.Insert(keys, i, k)
		}
	}
	i, found := slices.BinarySearch(keys, 30)
	fmt.Println("keys:", keys)
	fmt.Println("30 would go at index", i, "found:", found)
	if i < len(keys) {
		fmt.Println("ceiling(30):", keys[i])
	}
}
