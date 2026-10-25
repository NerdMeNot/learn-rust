package main

import "fmt"

type Key struct {
	Name string
	Tag  any
}

func main() {
	seen := map[Key]bool{}
	seen[Key{"a", 1}] = true // compiles: an interface value counts as comparable
	fmt.Println("int tag stored")
	seen[Key{"b", []int{1}}] = true
	fmt.Println("slice tag stored")
}
