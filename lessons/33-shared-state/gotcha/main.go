package main

import (
	"fmt"
	"sync"
)

func main() {
	var results []int
	var wg sync.WaitGroup
	for i := 0; i < 3; i++ {
		wg.Add(1)
		go func(i int) {
			defer wg.Done()
			results = append(results, i*10) // no lock!
		}(i)
	}
	wg.Wait()
	fmt.Println(len(results))
}
