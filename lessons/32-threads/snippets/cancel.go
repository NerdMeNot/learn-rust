package main

import (
	"context"
	"fmt"
)

func main() {
	ctx, cancel := context.WithCancel(context.Background())
	done := make(chan string)
	go func() {
		<-ctx.Done() // a real worker checks this between pieces of work
		done <- "goroutine: ctx.Err() is " + ctx.Err().Error()
	}()
	cancel()
	fmt.Println(<-done)
}
