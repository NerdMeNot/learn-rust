package main

import "fmt"

type Color int

const (
	Red   Color = iota // 0
	Green              // 1
	Blue               // 2
)

func (c Color) String() string {
	switch c {
	case Red:
		return "Red"
	case Green:
		return "Green"
	case Blue:
		return "Blue"
	}
	return "unknown"
}

func main() {
	fmt.Println("Green:", Green, "as int:", int(Green))
	var c Color = 42 // compiles: every int is a Color
	fmt.Println("Color(42):", c)
	var zero Color // the zero value is the first constant
	fmt.Println("zero value:", zero)
}
