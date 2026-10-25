package main

import "fmt"

func main() {
	nums := []int{1, 2, 3}
	for _, n := range nums {
		n *= 10 // n is a copy of the item
	}
	fmt.Println("after range with n *= 10:", nums)

	for i := range nums {
		nums[i] *= 10
	}
	fmt.Println("after nums[i] *= 10:      ", nums)

	for i, r := range "héllo" {
		fmt.Print(i, ":", string(r), " ")
	}
	fmt.Println()
}
