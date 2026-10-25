package main

import "fmt"

type Point struct{ x, y int }

func main() {
	p := Point{3, -2}
	x, y := p
	fmt.Println(x, y)
}
