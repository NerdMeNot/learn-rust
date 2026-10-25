package main

import "fmt"

// Robot is written first and knows nothing about Speaker.
type Robot struct{ ID int }

func (r Robot) Speak() string { return fmt.Sprintf("beep %d", r.ID) }

// Later, the code that needs a Speaker describes the method it calls.
type Speaker interface{ Speak() string }

var _ Speaker = Robot{} // optional: a compile-time check that Robot is a Speaker

func main() {
	var s Speaker = Robot{ID: 7}
	fmt.Println(s.Speak())
}
