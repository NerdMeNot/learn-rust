package main

import "fmt"

type Counter struct{ count int }

// A value receiver gets a copy of the struct.
func (c Counter) BumpCopy() { c.count++ }

// A pointer receiver gets the struct's address.
func (c *Counter) BumpInPlace() { c.count++ }

func main() {
	c := Counter{}
	c.BumpCopy()
	fmt.Println("after BumpCopy:", c.count)
	c.BumpInPlace()
	fmt.Println("after BumpInPlace:", c.count)
}
