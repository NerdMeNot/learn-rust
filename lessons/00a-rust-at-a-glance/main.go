package main

import "fmt"

// Greet someone, then add up 1 to 4.
func greet(name string) string {
	return fmt.Sprintf("Hello, %s!", name)
}

func add(a, b int) int {
	return a + b
}

func main() {
	fmt.Println(greet("Ferris"))
	total := 0
	for n := 1; n <= 4; n++ {
		total = add(total, n)
	}
	fmt.Printf("total: %d\n", total)
}
