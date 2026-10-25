package main

import (
	"fmt"
	"strconv"
)

func main() {
	var n int
	var s string
	fmt.Printf("zero values: %d %q\n", n, s)

	a, err := strconv.Atoi("1")
	b, err := strconv.Atoi("x") // b is new; err is reused, not redeclared
	fmt.Println(a, b, err)
}
