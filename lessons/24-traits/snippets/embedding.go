package main

import (
	"fmt"
	"strings"
)

type Base struct{}

func (Base) Speak() string { return "..." }

// Shout is built on Speak, as a default method would be.
func (b Base) Shout() string { return strings.ToUpper(b.Speak()) }

type Dog struct{ Base }

func (Dog) Speak() string { return "woof" }

func main() {
	d := Dog{}
	fmt.Println("d.Speak():", d.Speak())
	fmt.Println("d.Shout():", d.Shout()) // Base.Shout calls Base.Speak, not Dog.Speak
}
