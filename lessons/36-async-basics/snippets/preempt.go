package main

import (
	"fmt"
	"runtime"
	"sync/atomic"
	"time"
)

func main() {
	runtime.GOMAXPROCS(1) // one thread runs every goroutine
	var stop atomic.Bool
	go func() {
		for !stop.Load() { // never waits, never calls a function that could
		}
	}()
	time.Sleep(10 * time.Millisecond)
	fmt.Println("main got its turn while another goroutine was spinning")
	stop.Store(true)
}
