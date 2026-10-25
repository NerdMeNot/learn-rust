package main

import "fmt"

// T is one of these types, and `+` on a T gives a T.
func add[T int | float64](a, b T) T {
	return a + b
}

func main() {
	fmt.Println(add(2, 3))
	fmt.Println(add(1.5, 2.25))
}
