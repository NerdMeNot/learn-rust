package main

import "fmt"

type Counter struct{ count int }

func (c *Counter) Bump() { c.count++ }

func main() {
	c := Counter{}
	c.Bump()
	fmt.Println(c.count)
}
