package main

import (
	"fmt"
	"os"
	"sync"
)

func main() {
	var ready, done sync.WaitGroup
	release := make(chan struct{})
	for i := 0; i < 1000; i++ {
		ready.Add(1)
		done.Add(1)
		go func() {
			defer done.Done()
			ready.Done()
			<-release // blocked on a channel, not on the OS
		}()
	}
	ready.Wait()
	during, _ := os.ReadDir("/proc/self/task")
	fmt.Println("goroutines started: 1000")
	fmt.Println("OS threads fewer than the goroutines:", len(during) < 1000)
	close(release)
	done.Wait()
}
