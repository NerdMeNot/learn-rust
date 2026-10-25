package main

import (
	"fmt"
	"sync"
)

func main() {
	fmt.Println("start")
	// Go has no future value. A function value is the nearest thing:
	// holding it runs nothing until it is called.
	task := func() { fmt.Println("task body runs") }
	fmt.Println("task created")

	// `go` starts it running alongside main; WaitGroup waits for it.
	var wg sync.WaitGroup
	wg.Add(1)
	go func() {
		defer wg.Done()
		task()
	}()
	wg.Wait()
	fmt.Println("end")
}
