package main

import (
	"fmt"
	"sync"
)

func main() {
	words := []string{"alpha", "beta"}

	// hand the slice to another goroutine and get it back over a channel
	back := make(chan []string)
	go func() { back <- words }()
	words = <-back
	fmt.Printf("moved there and back: [%q, %q]\n", words[0], words[1])

	// two goroutines read the same slice at once; nothing checks that this is safe
	lens := make([]int, 2)
	var wg sync.WaitGroup
	for i := range lens {
		wg.Add(1)
		go func(i int) {
			defer wg.Done()
			lens[i] = len(words[i])
		}(i)
	}
	wg.Wait()
	fmt.Printf("lengths read in parallel: %d and %d\n", lens[0], lens[1])
}
