package main

import "fmt"

type Person struct {
	Name string
	Age  int
}

type Team struct {
	Name    string
	Members []string
}

func main() {
	// []Person: p is a copy of the struct
	people := []Person{{"Ada", 36}, {"Linus", 54}}
	for _, p := range people {
		p.Age++
	}
	fmt.Println("[]Person,  p.Age++:", people)

	// []*Person: p is a copy of the pointer, so it leads to the same Person
	ptrs := []*Person{{"Ada", 36}, {"Linus", 54}}
	for _, p := range ptrs {
		p.Age++
	}
	fmt.Println("[]*Person, p.Age++:", *ptrs[0], *ptrs[1])

	// The detector: is p the element itself?
	for i, p := range people {
		fmt.Println("[]Person  element", i, "same variable as p:", &p == &people[i])
	}
	for i, p := range ptrs {
		fmt.Println("[]*Person element", i, "same pointer as p: ", p == ptrs[i])
	}

	// A struct is copied whole, but a slice field inside it is copied as a header
	teams := []Team{{"red", []string{"ana"}}}
	for _, t := range teams {
		t.Name = "changed"
		t.Members[0] = "changed"
	}
	fmt.Println("[]Team, after t.Name and t.Members[0]:", teams)
}
