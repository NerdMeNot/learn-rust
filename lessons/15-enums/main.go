package main

import "fmt"

type Shape interface{}
type Circle struct{ R float64 }
type Rect struct{ W, H float64 }

func area(s Shape) float64 {
	switch v := s.(type) {
	case Circle:
		return 3.14 * v.R * v.R
	case Rect:
		return v.W * v.H
	}
	return 0
}

func main() {
	shapes := []Shape{Circle{1.0}, Rect{2.0, 3.0}}
	for _, s := range shapes {
		fmt.Println(area(s))
	}
}
