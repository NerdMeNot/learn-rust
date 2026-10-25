package main

import "fmt"

func try(name string, f func()) {
	defer func() { fmt.Printf("%s: %v\n", name, recover()) }()
	f()
}

func main() {
	ch := make(chan int, 2)
	ch <- 1
	ch <- 2
	close(ch) // the SENDER side closes; receivers can still drain what is queued

	for i := 0; i < 3; i++ {
		v, ok := <-ch
		fmt.Println("receive:", v, ok) // the third is a zero value, with ok == false
	}
	try("send on a closed channel", func() { ch <- 3 })
	try("close of a closed channel", func() { close(ch) })

	// select with a default case is a non-blocking send: Rust's try_send
	full := make(chan int, 1)
	full <- 1
	select {
	case full <- 2:
		fmt.Println("sent")
	default:
		fmt.Println("would block")
	}
}
