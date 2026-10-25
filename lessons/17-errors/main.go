package main

import (
	"fmt"
	"strconv"
)

func parseAge(s string) (int, error) {
	n, err := strconv.Atoi(s)
	if err != nil {
		return 0, err
	}
	return n, nil
}

func main() {
	for _, s := range []string{"42", "abc"} {
		if n, err := parseAge(s); err != nil {
			fmt.Println("error:", err)
		} else {
			fmt.Println("age", n)
		}
	}
}
