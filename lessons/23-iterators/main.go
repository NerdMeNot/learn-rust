package main

import "fmt"

func main() {
	nums := []int{1, 2, 3, 4, 5, 6}

	var evensSquared []int
	total := 0
	for _, n := range nums {
		if n%2 == 0 {
			evensSquared = append(evensSquared, n*n)
		}
		total += n
	}
	fmt.Println(evensSquared, total)
}
