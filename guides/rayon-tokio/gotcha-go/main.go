package main

import (
	"fmt"
	"runtime"
	"sync/atomic"
)

func main() {
	runtime.GOMAXPROCS(1) // one thread runs every goroutine, like the single-thread runtime

	var ticks atomic.Int32
	go func() { // the heartbeat
		for i := 0; i < 3; i++ {
			ticks.Add(1)
			runtime.Gosched()
		}
	}()

	// CPU-bound work in the main goroutine, with no function call that could yield.
	// It spins until the heartbeat has ticked (the cap is only a safety net).
	n := 0
	for ticks.Load() == 0 && n < 2_000_000_000 {
		n++
	}
	fmt.Println("the heartbeat ticked while the loop was still running:", ticks.Load() > 0)
}
