package main

import "fmt"

func add(a, b int) int { return a + b }

func pair() (int, int) { return 3, 4 }

func main() {
	// A call that returns exactly the parameters can be passed straight in.
	fmt.Println("add(pair()):", add(pair()))
}
