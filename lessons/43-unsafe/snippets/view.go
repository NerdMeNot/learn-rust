package main

import (
	"fmt"
	"unsafe"
)

func main() {
	s := "héllo"
	// a []byte that shares the string's memory instead of copying it
	b := unsafe.Slice(unsafe.StringData(s), len(s))
	fmt.Println("bytes:", b)
	fmt.Println("same address:", unsafe.StringData(s) == &b[0])
	fmt.Println("sizeof string header:", unsafe.Sizeof(s))
}
