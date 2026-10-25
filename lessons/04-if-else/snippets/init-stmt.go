package main

import (
	"fmt"
	"strconv"
)

func main() {
	// the init statement runs first, and `n` exists only inside the if/else
	if n, err := strconv.Atoi("42"); err != nil {
		fmt.Println("not a number:", err)
	} else {
		fmt.Println("number:", n)
	}
}
