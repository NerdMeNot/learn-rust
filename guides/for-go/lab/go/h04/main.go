package main

import "fmt"

type Speaker interface{ Speak() string }

type Robot struct{}

func (Robot) Speak() string { return "beep" }

func introduce(s Speaker) { fmt.Println(s.Speak()) }

func main() {
	introduce(Robot{}) // Robot has Speak, so it is a Speaker
}
