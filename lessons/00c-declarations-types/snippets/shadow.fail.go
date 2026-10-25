package main

import "fmt"

func main() {
	x := 1
	x := x * 10 // a second := in the same scope needs a new name
	fmt.Println(x)
}
