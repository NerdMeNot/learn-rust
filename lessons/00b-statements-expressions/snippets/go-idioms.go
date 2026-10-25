package main

import (
	"fmt"
	"unsafe"
)

func f() int { return 5 }

func named() (n int) {
	n = 3
	return
}

func main() {
	i := 0
	i++
	fmt.Println("i++:", i)
	if v := f(); v > 3 {
		fmt.Println("init statement:", v)
	}
	var u struct{}
	fmt.Println("struct{} size:", unsafe.Sizeof(u))
	fmt.Println("named result:", named())
}
