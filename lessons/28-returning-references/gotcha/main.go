package main

import "fmt"

func makeGreeting(name string) *string {
	s := "hello " + name
	return &s
}

func main() {
	fmt.Println(*makeGreeting("rust"))
}
