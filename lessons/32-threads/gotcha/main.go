package main

import (
	"fmt"
	"sync"
)

func main() {
	names := []string{"a", "b", "c"}
	var wg sync.WaitGroup
	wg.Add(1)
	go func() {
		defer wg.Done()
		fmt.Println(len(names))
	}()
	wg.Wait()
}
