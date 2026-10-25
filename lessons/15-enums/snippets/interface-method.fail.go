package main

import "fmt"

type Shape interface{ Area() float64 }

type Circle struct{ R float64 }

func (c Circle) Area() float64 { return 3.14 * c.R * c.R }

type Triangle struct{ B, H float64 } // new, and no Area method yet

func main() {
	shapes := []Shape{Circle{1.0}, Triangle{2.0, 3.0}}
	for _, s := range shapes {
		fmt.Println(s.Area())
	}
}
