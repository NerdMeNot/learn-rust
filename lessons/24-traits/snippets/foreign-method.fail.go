package main

import "fmt"

type Speaker interface{ Speak() string }

func (i int) Speak() string { return fmt.Sprintf("the number %d", i) }

func main() {
	fmt.Println("unreachable: the file does not compile")
}
