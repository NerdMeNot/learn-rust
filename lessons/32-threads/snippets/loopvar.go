package main

import (
	"fmt"
	"sync"
)

func main() {
	results := make([]int, 4)
	var wg sync.WaitGroup
	for i := 0; i < 4; i++ {
		wg.Go(func() { results[i] = i * i }) // since Go 1.22 each iteration has its own i
	}
	wg.Wait()
	fmt.Println("results:", results)
}
