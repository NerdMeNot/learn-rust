package main

import "fmt"

func main() {
	for _, n := range []int{1, 5} {
		switch n {
		default:
			fmt.Println(n, "default")
		case 1:
			fmt.Println(n, "one")
		}
	}
}
