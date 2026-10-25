package main

import "fmt"

func main() {
	started := make(chan struct{})
	block := make(chan struct{}) // nobody ever sends or closes this
	go func() {
		defer fmt.Println("goroutine: deferred call ran")
		close(started)
		<-block
	}()
	<-started
	fmt.Println("main: the goroutine is running, and main returns now")
}
