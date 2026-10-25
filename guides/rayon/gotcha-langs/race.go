package main

import (
	"fmt"
	"runtime"
	"sync"
)

func main() {
	const n = 1_000_000
	parts := runtime.NumCPU()
	total := 0 // shared by every goroutine
	var wg sync.WaitGroup
	for p := 0; p < parts; p++ {
		lo, hi := n*p/parts+1, n*(p+1)/parts
		wg.Add(1)
		go func() {
			defer wg.Done()
			for i := lo; i <= hi; i++ {
				total += i
			}
		}()
	}
	wg.Wait()
	fmt.Println("total:", total, "expected: 500000500000")
}
