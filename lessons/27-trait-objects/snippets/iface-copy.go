package main

import "fmt"

type Valuer interface{ Value() int }

type Counter struct{ N int }

func (c Counter) Value() int { return c.N }

func main() {
	c := Counter{1}
	var v Valuer = c // the interface gets a copy of c
	c.N = 99
	fmt.Println("c.N:", c.N)
	fmt.Println("the interface sees:", v.Value())
}
