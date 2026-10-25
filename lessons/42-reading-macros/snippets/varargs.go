package main

import "fmt"

func sum(xs ...int) int {
	total := 0
	for _, x := range xs {
		total += x
	}
	return total
}

func main() {
	fmt.Println("sum() =", sum())
	fmt.Println("sum(1, 2, 3, 4) =", sum(1, 2, 3, 4))
}
