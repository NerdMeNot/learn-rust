package main

import (
	"context"
	"fmt"
	"sync"
)

func main() {
	ctx, cancel := context.WithCancel(context.Background())
	cancel() // cancelled before either worker starts

	var wg sync.WaitGroup
	var checks, ignores int
	wg.Add(2)
	go func() { // looks at ctx before each step
		defer wg.Done()
		for i := 0; i < 3 && ctx.Err() == nil; i++ {
			checks++
		}
	}()
	go func() { // never looks
		defer wg.Done()
		for i := 0; i < 3; i++ {
			ignores++
		}
	}()
	wg.Wait()
	fmt.Println("steps done by the worker that checks ctx:", checks)
	fmt.Println("steps done by the worker that ignores it: ", ignores)
}
