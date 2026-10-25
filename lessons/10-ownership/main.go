package main

import "fmt"

func main() {
	a := []int{1, 2, 3}
	b := a         // a and b now share the SAME array
	b[0] = 100     // change through b...
	fmt.Println(a) // ...and a changed too: [100 2 3]
	fmt.Println(b)

	x := 5
	y := x
	fmt.Println(x, y)
}
