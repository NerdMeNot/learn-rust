package main

import "fmt"

func main() {
	for _, n := range []int{1, 2, 3} {
		switch n {
		case 1:
			fmt.Println(n, "case 1")
			fallthrough
		case 2:
			fmt.Println(n, "case 2")
		default:
			fmt.Println(n, "default")
		}
	}
}
