package main

import "fmt"

func main() {
	passes := 0
	for i := 0; i < 5; i++ {
		if i == 1 {
			i += 2
		}
		passes++
		fmt.Println("i:", i)
	}
	fmt.Println("passes:", passes)
}
