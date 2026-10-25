package main

import (
	"fmt"
	"sync"
)

func main() {
	var wg sync.WaitGroup
	count := 0
	for range 2 {
		wg.Add(1)
		go func() {
			defer wg.Done()
			for range 1_000_000 {
				count++
			}
		}()
	}
	wg.Wait()
	fmt.Println(count)
}
