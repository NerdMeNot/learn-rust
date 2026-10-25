package main

import "fmt"

type Counter struct {
	count int
}

func NewCounter() *Counter {
	return &Counter{}
}

func (c *Counter) Get() int {
	return c.count
}

func (c *Counter) Bump() {
	c.count++
}

func main() {
	c := NewCounter()
	c.Bump()
	c.Bump()
	fmt.Println(c.Get())
}
