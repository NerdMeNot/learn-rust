package main

import "fmt"

func main() {
	count := 0
	inc := func() { count++ }
	inc()
	inc()
	fmt.Printf("count=%d\n", count)
}
