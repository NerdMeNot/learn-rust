package main

import (
	"fmt"
	"runtime"
	"sync"
)

func main() {
	const n = 1_000_000
	parts := runtime.NumCPU()
	partial := make([]uint64, parts) // one slot per goroutine, so no sharing
	var wg sync.WaitGroup
	for p := 0; p < parts; p++ {
		lo, hi := n*p/parts+1, n*(p+1)/parts
		wg.Add(1)
		go func() {
			defer wg.Done()
			for i := lo; i <= hi; i++ {
				partial[p] += uint64(i) * uint64(i)
			}
		}()
	}
	wg.Wait()
	var total uint64
	for _, s := range partial {
		total += s
	}
	fmt.Println(total)
}
