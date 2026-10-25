package main

import (
	"fmt"
	"unsafe"
)

type Shape interface{ Area() int }

type Square struct{ Side int }

func (s Square) Area() int { return s.Side * s.Side }

func main() {
	var s Shape = Square{3}
	var p *Square = &Square{3}
	var e Shape // an interface value holding nothing
	fmt.Println("size of an interface value:", unsafe.Sizeof(s))
	fmt.Println("size of a *Square:", unsafe.Sizeof(p))
	fmt.Println("empty interface is nil:", e == nil)
}
