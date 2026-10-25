package main

import "fmt"

func area(width int, height int = 1) int {
	return width * height
}

func main() {
	fmt.Println(area(3))
}
