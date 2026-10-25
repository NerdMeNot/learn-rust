package main

import "fmt"

func main() {
	factor := 3
	times := func(x int) int { return x * factor }
	fmt.Println(times(5))

	count := 0
	inc := func() { count++ }
	inc()
	inc()
	fmt.Printf("count=%d\n", count)
}
