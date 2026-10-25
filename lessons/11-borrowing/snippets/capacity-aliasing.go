package main

import "fmt"

// grow appends and then writes to the first element.
func grow(xs []int) {
	xs = append(xs, 4)
	xs[0] = 99
}

func main() {
	full := make([]int, 3, 3)   // no spare room
	roomy := make([]int, 3, 10) // room for more
	grow(full)
	grow(roomy)
	fmt.Println("full: ", full, " cap", cap(full))
	fmt.Println("roomy:", roomy, "cap", cap(roomy))
}
