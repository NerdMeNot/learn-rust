package main

import "fmt"

func logMsg(msg string) {
	fmt.Println("log: " + msg)
}

func main() {
	n := 7

	size := "small"
	if n > 5 {
		size = "big"
	}
	fmt.Println("ternary:", size)

	area := func() int {
		w := 3
		h := 4
		return w * h
	}()
	fmt.Println("block:", area)

	name := ""
	switch 2 {
	case 1:
		name = "one"
	case 2:
		name = "two"
	default:
		name = "many"
	}
	fmt.Println("switch:", name)

	a, b := 1, 2
	a, b = b, a
	fmt.Println("swap:", a, b)

	logMsg("first")
	c := 2
	fmt.Println("comma:", c)

	addOne := func(x int) int {
		y := x + 1
		return y
	}
	fmt.Println("closure:", addOne(41))
}
