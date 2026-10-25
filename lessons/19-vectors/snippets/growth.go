package main

import (
	"fmt"
	"runtime"
	"unsafe"
)

func firstAppend[T any](name string) {
	var s []T
	s = append(s, *new(T))
	fmt.Printf("first append to a []%s: cap %d\n", name, cap(s))
}

func main() {
	fmt.Println(runtime.Version())
	var v []int
	fmt.Println("size of a slice header:", unsafe.Sizeof(v), "bytes")
	fmt.Printf("start: len %d, cap %d\n", len(v), cap(v))
	last := cap(v)
	for i := 1; i <= 40; i++ {
		v = append(v, i)
		if cap(v) != last {
			fmt.Printf("append #%d: cap %d -> %d\n", i, last, cap(v))
			last = cap(v)
		}
	}
	firstAppend[string]("string")
	firstAppend[byte]("byte")
}
