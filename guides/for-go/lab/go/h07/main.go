package main

import "fmt"

type Animal struct{ Name string }

func (a Animal) Describe() string { return a.Name + " is an animal" }

type Dog struct {
	Animal
	Tricks int
}

func main() {
	d := Dog{Animal{"rex"}, 2}
	fmt.Println(d.Name, "|", d.Describe())
}
