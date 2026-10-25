package main

import "fmt"

func main() {
	n := 0
	for i := 0; i < 5; i++ {
		switch i {
		case 2:
			break
		default:
			n++
		}
	}
	fmt.Printf("n: %d\n", n)
}
