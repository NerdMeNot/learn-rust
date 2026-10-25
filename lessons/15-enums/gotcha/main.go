package main

import "fmt"

type Circle struct{ R float64 }
type Rect struct{ W, H float64 }
type Triangle struct{ B, H float64 } // new!

func area(s any) float64 {
	switch v := s.(type) {
	case Circle:
		return 3.14 * v.R * v.R
	case Rect:
		return v.W * v.H
	}
	return 0
}

func main() {
	fmt.Println(area(Triangle{2.0, 3.0}))
}
