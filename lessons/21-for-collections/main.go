package main

import "fmt"

func main() {
	names := []string{"ana", "bo"}
	for _, n := range names {
		fmt.Println("hi", n)
	}

	nums := []int{1, 2, 3}
	for i := range nums {
		nums[i] *= 10
	}

	for i, n := range nums {
		fmt.Printf("%d: %d\n", i, n)
	}
	fmt.Println(len(names), "names,", nums)
}
