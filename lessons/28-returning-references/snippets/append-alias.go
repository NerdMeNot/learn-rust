package main

import "fmt"

// returns a piece of the input: no copy
func firstTwo(xs []int) []int {
	return xs[:2]
}

// the same piece, with its capacity cut to its length
func firstTwoCapped(xs []int) []int {
	return xs[:2:2]
}

func main() {
	scores := []int{3, 9, 4}
	top := append(firstTwo(scores), 99)
	fmt.Println("top:   ", top)
	fmt.Println("scores:", scores)

	scores = []int{3, 9, 4}
	capped := append(firstTwoCapped(scores), 99)
	fmt.Println("capped:", capped)
	fmt.Println("scores:", scores)
}
