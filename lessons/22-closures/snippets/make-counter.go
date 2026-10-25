package main

import "fmt"

func makeCounter() func() int {
	count := 0
	return func() int {
		count++
		return count
	}
}

func main() {
	a := makeCounter()
	b := makeCounter()
	fmt.Println("a:", a(), a(), "b:", b())
}
