package main

import "fmt"

func main() {
	ch := make(chan int)
	go func() {
		for i := 1; i <= 2; i++ {
			ch <- i
		}
		// forgot close(ch)
	}()
	for v := range ch {
		fmt.Println("got", v)
	}
}
