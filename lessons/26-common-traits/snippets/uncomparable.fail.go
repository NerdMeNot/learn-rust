package main

import "fmt"

type Release struct {
	Name string
	Tags []string
}

func main() {
	seen := map[Release]bool{}
	fmt.Println(len(seen))

	a := Release{"1.0", nil}
	b := Release{"1.0", nil}
	fmt.Println(a == b)
}
