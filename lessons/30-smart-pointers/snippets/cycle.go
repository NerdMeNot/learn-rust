package main

import (
	"fmt"
	"runtime"
	"weak"
)

type Node struct{ other *Node }

func makeCycle() weak.Pointer[Node] {
	a := &Node{}
	b := &Node{}
	a.other = b
	b.other = a
	return weak.Make(a) // does not keep a alive
}

func main() {
	w := makeCycle()
	runtime.GC()
	fmt.Println("cycle collected:", w.Value() == nil)
}
