package main

import "fmt"

func main() {
	names := []string{"ana", "bo"}
	for _, n := range names {
		fmt.Println("hi", n)
	}
	fmt.Println(len(names), "names")
}
