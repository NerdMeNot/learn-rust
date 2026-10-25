package main

import "fmt"

func newCounter() *int {
	n := 0
	return &n // n must outlive this call, so Go keeps it alive on the heap
}

func main() {
	p := newCounter()
	*p += 5
	fmt.Println("*p:", *p)
}
