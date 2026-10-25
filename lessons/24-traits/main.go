package main

import (
	"fmt"
	"strings"
)

type Speaker interface {
	Speak() string
}

type Dog struct{}
type Robot struct{ ID int }

func (Dog) Speak() string     { return "woof" }
func (r Robot) Speak() string { return fmt.Sprintf("beep %d", r.ID) }

func shout(s Speaker) string { return strings.ToUpper(s.Speak()) }

func introduce(s Speaker) {
	fmt.Println(s.Speak(), "/", shout(s))
}

func main() {
	introduce(Dog{})
	introduce(Robot{ID: 7})
}
