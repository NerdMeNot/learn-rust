package main

import (
	"fmt"
	"sync"
)

func main() {
	var wg sync.WaitGroup
	wg.Add(1)
	go func() {
		var v []int
		fmt.Println("goroutine: about to index an empty slice")
		fmt.Println(v[0]) // panics here, so the next line never runs
		wg.Done()
	}()
	wg.Wait()
	fmt.Println("main: never reached")
}
