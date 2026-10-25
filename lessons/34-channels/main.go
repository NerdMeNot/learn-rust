package main

import "fmt"

func main() {
	ch := make(chan string)

	go func() {
		for i := 1; i <= 3; i++ {
			ch <- fmt.Sprintf("job %d", i)
		}
		close(ch)
	}()

	for msg := range ch {
		fmt.Println("got", msg)
	}
}
