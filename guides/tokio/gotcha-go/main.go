package main

import (
	"fmt"
	"sync"
	"time"
)

func main() {
	var mu sync.Mutex
	hits := 0
	done := make(chan bool)
	go func() {
		mu.Lock()
		time.Sleep(10 * time.Millisecond) // still holding the lock
		hits++
		mu.Unlock()
		done <- true
	}()
	<-done
	fmt.Println(hits)
}
