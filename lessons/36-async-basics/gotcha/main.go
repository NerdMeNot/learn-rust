package main

import "fmt"

// Go has no async functions: a call that waits is an ordinary call.
func loadCount() int {
	return 3
}

func report() int {
	n := loadCount()
	return n + 1
}

func main() {
	fmt.Println("count:", report())
}
