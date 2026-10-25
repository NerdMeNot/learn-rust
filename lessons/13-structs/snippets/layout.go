package main

import (
	"fmt"
	"unsafe"
)

type S struct {
	A uint8
	B uint32
	C uint8
}

func main() {
	var s S
	fmt.Println("S: size", unsafe.Sizeof(s), "align", unsafe.Alignof(s))
	fmt.Println("  offsets: A", unsafe.Offsetof(s.A), "B", unsafe.Offsetof(s.B), "C", unsafe.Offsetof(s.C))
}
