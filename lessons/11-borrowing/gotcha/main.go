package main

import "fmt"

func total(xs []int) int {
	sum := 0
	for _, x := range xs {
		sum += x
	}
	return sum
}

func main() {
	nums := []int{1, 2, 3}
	t := total(nums)
	fmt.Println(t, nums)
}
