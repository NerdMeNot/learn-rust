package main

import (
	"fmt"
	"sync"
)

func main() {
	names := []string{"a", "b", "c"}
	var count int
	var wg sync.WaitGroup

	wg.Add(1)
	go func() {
		defer wg.Done()
		for _, n := range names {
			fmt.Println("worker sees", n)
		}
		count = len(names)
	}()

	wg.Wait()
	fmt.Println("worker counted", count)
}
