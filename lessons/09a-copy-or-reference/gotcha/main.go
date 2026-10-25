package main

import "fmt"

func main() {
	ages := []int{30, 40, 50}
	for _, age := range ages {
		age++
	}
	fmt.Println(ages)
}
