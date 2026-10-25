package main

import "fmt"

func main() {
	total := 0
	for n := 1; n <= 4; n++ {
		total += n
	}
	fmt.Printf("total: %d\n", total)
}
