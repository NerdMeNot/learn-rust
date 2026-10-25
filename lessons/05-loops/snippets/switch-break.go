package main

import "fmt"

func main() {
	for n := 1; n <= 5; n++ {
		switch n {
		case 3:
			break // leaves the switch only
		default:
			fmt.Println("n:", n)
		}
	}
	fmt.Println("after the loop")
}
