package main

import (
	"fmt"
	"unsafe"
)

func main() {
	prices := []int{10, 20, 30}
	p := unsafe.Pointer(&prices[0])
	second := *(*int)(unsafe.Add(p, unsafe.Sizeof(prices[0])))
	fmt.Printf("second: %d\n", second)
}
