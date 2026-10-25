package main

import (
	"fmt"
	"sync"
)

func main() {
	name := "Ferris"
	count := 0
	var wg sync.WaitGroup
	wg.Add(1)
	// a goroutine's closure may use any local, and change it
	go func() {
		defer wg.Done()
		count++
		fmt.Println("hello,", name)
	}()
	wg.Wait()
	fmt.Println("count:", count)
}
