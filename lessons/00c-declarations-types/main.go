package main

import (
	"fmt"
	"strconv"
)

const limit uint32 = 1_000

func half(x float64) float64 {
	return x / 2.0
}

func main() {
	var name string = "Ada"
	var visits uint8 = 0
	visits++
	mask, ratio := 0xff, float32(2.5)
	letter := 'R'
	total := float64(limit) + half(5.0)
	parsed, _ := strconv.Atoi("42")

	fmt.Printf("name=%s visits=%d\n", name, visits)
	fmt.Printf("mask=%d ratio=%v letter=%c\n", mask, ratio, letter)
	fmt.Printf("limit=%d total=%v\n", limit, total)
	fmt.Printf("parsed=%d\n", parsed)
}
