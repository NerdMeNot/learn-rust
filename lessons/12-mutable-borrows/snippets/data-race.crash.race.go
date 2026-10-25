package main

import (
	"fmt"
	"time"
)

func main() {
	s := []int{0}
	go func() {
		s[0] = 1 // one goroutine writes the element...
	}()
	time.Sleep(100 * time.Millisecond) // (no lock or channel: only a delay)
	s[0] = 2                           // ...and main writes it, with nothing in between
	fmt.Println("s:", s)
}
