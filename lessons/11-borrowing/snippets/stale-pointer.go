package main

import "fmt"

func main() {
	a := []int{7, 8, 9}
	p := &a[0]
	a = append(a, 10) // a is full, so append moves its data to a new array
	*p = 100          // ...but p still points into the old one
	fmt.Println("a[0] =", a[0], " *p =", *p)
}
