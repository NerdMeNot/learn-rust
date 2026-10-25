package main

import (
	"fmt"
	"math"
	"strconv"
)

func main() {
	fmt.Println("int bits:", strconv.IntSize)
	fmt.Println("math.MaxInt:", math.MaxInt)

	var small int32 = math.MaxInt32
	small++
	fmt.Println("MaxInt32 + 1:", small)

	n := 300
	fmt.Println("uint8(n):", uint8(n))

	s := "hello"
	fmt.Printf("len(s) has type %T\n", len(s))

	price := 2.5
	fmt.Println("price * 2:", price*2)

	a, b := -7, 2
	fmt.Println("-7 / 2:", a/b)
	fmt.Println("-7 % 2:", a%b)
}
