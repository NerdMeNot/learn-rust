package main

import "fmt"

func main() {
	x := 1
	show := func() { fmt.Println("x:", x) }
	x = 2
	show()
}
