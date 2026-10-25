package main

import (
	"container/heap"
	"fmt"
)

// Go's heap is an interface you implement on your own type: Len, Less, Swap, Push, Pop.
type IntHeap []int

func (h IntHeap) Len() int           { return len(h) }
func (h IntHeap) Less(i, j int) bool { return h[i] < h[j] } // < makes a min-heap
func (h IntHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h *IntHeap) Push(x any)        { *h = append(*h, x.(int)) }
func (h *IntHeap) Pop() any {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[:n-1]
	return x
}

func main() {
	h := &IntHeap{3, 9, 4, 1}
	heap.Init(h)
	heap.Push(h, 7)
	fmt.Print("pop order:")
	for h.Len() > 0 {
		fmt.Print(" ", heap.Pop(h))
	}
	fmt.Println()

	// Go can change an item in place and restore the order with Fix; Rust's BinaryHeap cannot.
	h2 := &IntHeap{5, 2, 8}
	heap.Init(h2)
	(*h2)[0] = 9 // the smallest, 2, becomes 9
	heap.Fix(h2, 0)
	fmt.Println("after changing the top and calling Fix, pop:", heap.Pop(h2))
}
