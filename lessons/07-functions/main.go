package main

import "fmt"

func add(a, b int) int {
	return a + b
}

func greet(name string) {
	fmt.Println("hi", name)
}

func main() {
	fmt.Println(add(2, 3))
	greet("kotlin")
}
