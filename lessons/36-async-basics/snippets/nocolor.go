package main

import (
	"fmt"
	"io"
	"os"
	"sync"
	"time"
)

// An ordinary function. It waits for data to arrive, but nothing in its signature says so.
func fetchName(r io.Reader) string {
	data, _ := io.ReadAll(r)
	return string(data)
}

// A slow source: the name arrives after a delay.
func slowSource(name string) io.Reader {
	r, w, _ := os.Pipe()
	go func() {
		time.Sleep(10 * time.Millisecond)
		w.WriteString(name)
		w.Close()
	}()
	return r
}

func main() {
	fmt.Println("called directly:", fetchName(slowSource("ferris")))

	// the same function, run as a goroutine
	var wg sync.WaitGroup
	var got string
	src := slowSource("ferris")
	wg.Add(1)
	go func() {
		defer wg.Done()
		got = fetchName(src)
	}()
	wg.Wait()
	fmt.Println("run with go:", got)
}
