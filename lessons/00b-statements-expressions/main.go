package main

import "fmt"

func clamp(x int) int {
	if x < 0 {
		return 0
	}
	if x > 100 {
		return 100
	}
	return x
}

func main() {
	n := 7
	label := "odd"
	if n%2 == 0 {
		label = "even"
	}
	w := 3
	h := 4
	area := w * h
	fmt.Println(n, "is", label)
	fmt.Println("area:", area)
	fmt.Println("clamp:", clamp(-5), clamp(50), clamp(250))
}
