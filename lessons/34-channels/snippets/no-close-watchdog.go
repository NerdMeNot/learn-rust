package main

import (
	"fmt"
	"os"
	"time"
)

func main() {
	ch := make(chan int)
	go func() {
		for i := 1; i <= 2; i++ {
			ch <- i
		}
		// forgot close(ch)
	}()
	// Stands in for any other goroutine that can still run (a timer, a server loop).
	go func() {
		time.Sleep(300 * time.Millisecond)
		fmt.Println("300 ms later main is still in its range loop, and Go has reported nothing")
		os.Exit(0)
	}()
	for v := range ch {
		fmt.Println("got", v)
	}
}
