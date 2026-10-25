package main

import "fmt"

func main() {
	var a int32 = 7
	var b int64 = 10
	price := 2.5
	ok := true

	total := int64(a) + b
	cost := float64(a) * price
	fmt.Println(total, cost, ok)
}
