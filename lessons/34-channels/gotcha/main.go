package main

import (
	"fmt"
	"time"
)

func main() {
	ch := make(chan []int)
	done := make(chan bool)
	go func() {
		got := <-ch
		time.Sleep(10 * time.Millisecond)
		fmt.Println("receiver sees", got)
		done <- true
	}()
	msg := []int{1, 2}
	ch <- msg
	msg[0] = 99 // sender keeps changing what it already sent
	<-done
}
