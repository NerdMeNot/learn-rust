package main

import (
	"fmt"
	"unsafe"
)

func get(a []int, i int) string {
	if i < len(a) {
		return fmt.Sprintf("Some(%d)", a[i])
	}
	return "None"
}

func main() {
	prices := []int{10, 20, 30}
	p := unsafe.Pointer(&prices[0])
	second := *(*int)(unsafe.Add(p, unsafe.Sizeof(prices[0])))
	fmt.Printf("second, read by offset: %d\n", second)
	fmt.Printf("get(1): %s\n", get(prices, 1))
	fmt.Printf("get(9): %s\n", get(prices, 9))
}
