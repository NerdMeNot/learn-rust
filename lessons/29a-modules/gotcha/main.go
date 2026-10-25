package main

import "fmt"

func discountPercent(qty int) int {
	if qty >= 10 {
		return 20
	}
	return 0
}

func Total(qty int) int {
	full := qty * 10
	return full - full*discountPercent(qty)/100
}

func main() {
	fmt.Println("total:", Total(12))
	fmt.Printf("discount: %d%%\n", discountPercent(12))
}
