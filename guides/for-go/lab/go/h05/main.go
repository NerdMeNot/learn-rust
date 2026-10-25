package main

import "fmt"

func main() {
	a := []int{1, 2, 3, 4}
	b := a[:2]
	b = append(b, 99) // there is room in a's array
	fmt.Println(a, b)
}
