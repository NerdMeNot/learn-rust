package main

import (
	"fmt"
	"sort"
	"sync"
)

func main() {
	var mu sync.Mutex
	var results []int
	var wg sync.WaitGroup

	for i := 0; i < 3; i++ {
		wg.Add(1)
		go func(i int) {
			defer wg.Done()
			mu.Lock()
			results = append(results, i*10)
			mu.Unlock()
		}(i)
	}
	wg.Wait()

	sort.Ints(results)
	fmt.Println(results)
}
