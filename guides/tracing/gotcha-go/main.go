package main

import "fmt"

type Span struct{ name string }

func Start(name string) *Span {
	fmt.Println("enter", name)
	return &Span{name}
}

func (s *Span) End() { fmt.Println("exit", s.name) }

func handle() {
	span := Start("request")
	defer span.End() // runs when handle returns
	fmt.Println("listed notes")
}

func main() {
	handle()
	_ = Start("forgotten") // `_` throws the *Span away; nothing ever calls End
	fmt.Println("after")
}
