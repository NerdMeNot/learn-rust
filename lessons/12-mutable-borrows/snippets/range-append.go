package main

import "fmt"

func main() {
	s := []int{1, 2, 3, 4}
	var seen []int
	for _, x := range s { // range reads the slice once, before the loop starts
		seen = append(seen, x)
		if x == 2 {
			s = append(s, 99)
		}
	}
	fmt.Println("saw", seen, "slice is", s)
}
