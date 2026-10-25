package main

import (
	"context"
	"fmt"
	"sync"
	"time"
)

func main() {
	// The receiver takes one value and leaves. Go reports nothing: the sender just blocks.
	ch := make(chan int)
	finished := make(chan struct{})
	go func() {
		ch <- 1
		ch <- 2 // nobody will ever receive this
		close(finished)
	}()
	fmt.Println("received:", <-ch)
	select {
	case <-finished:
		fmt.Println("sender finished")
	case <-time.After(300 * time.Millisecond):
		fmt.Println("after 300 ms the sender is still blocked on its second send")
	}

	// The usual answer is a context: the sender selects on ctx.Done() as well.
	ctx, cancel := context.WithCancel(context.Background())
	out := make(chan int)
	sent := make(chan int)
	go func() {
		n := 0
		for {
			select {
			case out <- n:
				n++
			case <-ctx.Done():
				sent <- n
				return
			}
		}
	}()
	fmt.Println("received:", <-out)
	cancel() // the receiver hangs up
	fmt.Println("sends completed before cancel:", <-sent)

	// close(quit) wakes every receiver at once: a broadcast.
	quit := make(chan struct{})
	var wg sync.WaitGroup
	for i := 0; i < 3; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			<-quit
		}()
	}
	close(quit)
	wg.Wait()
	fmt.Println("close(quit) woke all 3 receivers")
}
