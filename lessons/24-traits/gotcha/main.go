package main

import "fmt"

type Speaker interface{ Speak() string }

type Robot struct{ ID int }

func (r Robot) Speak() string { return fmt.Sprintf("beep %d", r.ID) }

func introduce(s Speaker) { fmt.Println(s.Speak()) }

func main() {
	introduce(Robot{ID: 7})
}
