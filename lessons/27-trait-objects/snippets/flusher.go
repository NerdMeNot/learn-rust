package main

import "fmt"

type Writer interface{ Write(s string) }

type Flusher interface{ Flush() }

type File struct{}

func (File) Write(s string) { fmt.Print(s) }
func (File) Flush()         { fmt.Println("[flushed]") }

func main() {
	var w Writer = File{}
	// asks the value inside w whether it ALSO has the methods of another interface
	if f, ok := w.(Flusher); ok {
		w.Write("hi ")
		f.Flush()
	}
}
