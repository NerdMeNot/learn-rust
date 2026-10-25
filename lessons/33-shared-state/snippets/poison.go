package main

import (
	"fmt"
	"sync"
)

func main() {
	var mu sync.Mutex
	data := []int{1, 2, 3}
	done := make(chan struct{})

	go func() {
		defer close(done)
		defer func() { fmt.Println("worker died:", recover()) }()
		mu.Lock()
		defer mu.Unlock()
		data = append(data, 4)
		panic("worker failed halfway")
	}()
	<-done

	mu.Lock() // not poisoned: the deferred Unlock ran
	fmt.Println("main got the lock; data:", data)
	mu.Unlock()
}
