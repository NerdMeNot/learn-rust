package main

import "fmt"

func main() {
	n := 7
	if n%2 == 0 {
		fmt.Println(n, "is even")
	} else {
		fmt.Println(n, "is odd")
	}

	size := "small"
	if n > 5 {
		size = "big"
	}
	fmt.Println(size)
}
