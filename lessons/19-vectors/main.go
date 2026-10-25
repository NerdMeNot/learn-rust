package main

import "fmt"

func main() {
	var v []int
	v = append(v, 10)
	v = append(v, 20)

	w := []int{1, 2, 3}
	w = append(w, 4)

	part := w[1:3] // a window onto w's array, no copy
	fmt.Printf("%v %v %v len=%d\n", v, w, part, len(w))
}
