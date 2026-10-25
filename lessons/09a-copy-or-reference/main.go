package main

import "fmt"

func main() {
	ages := []int{30, 40, 50}

	// A copy: range gives you each element's value, not the element.
	for _, age := range ages {
		age++
		fmt.Println("the copy is now", age)
	}
	fmt.Println("ages after the copy loop:    ", ages)

	// The elements themselves: index into the slice.
	for i := range ages {
		ages[i]++
	}
	fmt.Println("ages after the in-place loop:", ages)
}
