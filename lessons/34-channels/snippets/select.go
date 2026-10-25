package main

import (
	"fmt"
	"sort"
	"sync"
	"time"
)

func main() {
	ch := make(chan int)

	// a timeout on a receive: Rust's recv_timeout
	select {
	case v := <-ch:
		fmt.Println("got", v)
	case <-time.After(50 * time.Millisecond):
		fmt.Println("timed out")
	}

	// select with a default case is a non-blocking receive: Rust's try_recv
	select {
	case v := <-ch:
		fmt.Println("got", v)
	default:
		fmt.Println("nothing ready")
	}

	// several consumers on one channel: each value goes to exactly one of them
	jobs := make(chan int)
	taken := make(chan int, 6)
	var workers sync.WaitGroup
	for w := 0; w < 3; w++ {
		workers.Add(1)
		go func() {
			defer workers.Done()
			for j := range jobs {
				taken <- j
			}
		}()
	}
	for j := 0; j < 6; j++ {
		jobs <- j
	}
	close(jobs)
	workers.Wait()
	close(taken)
	var got []int
	for j := range taken {
		got = append(got, j)
	}
	sort.Ints(got)
	fmt.Println("3 goroutines on one channel took", got)

	// a nil channel is never ready, so a select case can be switched off
	var off chan int
	select {
	case v := <-off:
		fmt.Println("got", v)
	default:
		fmt.Println("a nil channel never fires")
	}

	// WaitGroup plus a closer goroutine: close(out) once every producer is done
	out := make(chan int)
	var wg sync.WaitGroup
	for id := 1; id <= 3; id++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			out <- id * 10
		}()
	}
	go func() {
		wg.Wait()
		close(out)
	}()
	var all []int
	for v := range out {
		all = append(all, v)
	}
	sort.Ints(all)
	fmt.Println("fan-in:", all)
}
