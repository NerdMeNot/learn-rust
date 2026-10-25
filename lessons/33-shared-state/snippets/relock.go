package main

import (
	"fmt"
	"sync"
)

func main() {
	var mu sync.Mutex
	mu.Lock()
	fmt.Println("TryLock while this goroutine holds the lock:", mu.TryLock()) // the same goroutine asks again
	mu.Unlock()
	fmt.Println("after Unlock:", mu.TryLock())
}
