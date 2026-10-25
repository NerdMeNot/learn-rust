package main

import (
	"fmt"
	"sync"
	"sync/atomic"
)

func main() {
	var mu sync.Mutex
	count := 0 // protected by mu, by convention only
	var hits atomic.Int64
	var wg sync.WaitGroup
	for g := 0; g < 8; g++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			for i := 0; i < 1000; i++ {
				mu.Lock()
				count++
				mu.Unlock()
				hits.Add(1)
			}
		}()
	}
	wg.Wait()
	fmt.Println("count:", count, "hits:", hits.Load())
}
