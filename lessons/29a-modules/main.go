package main

import "fmt"

const price = 10

// Lowercase: visible only inside this package.
func discountPercent(qty int) int {
	if qty >= 10 {
		return 20
	}
	return 0
}

// Uppercase: visible to other packages.
func Total(qty int) int {
	full := qty * price
	return full - full*discountPercent(qty)/100
}

func main() {
	fmt.Println("3 items:", Total(3))
	fmt.Println("12 items:", Total(12))
}
