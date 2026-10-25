package main

import (
	"fmt"
	"strconv"
)

func main() {
	n, _ := strconv.Atoi("abc")
	fmt.Println(n + 1)

	m, err := strconv.Atoi("abc")
	fmt.Println(m, err)
}
