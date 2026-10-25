package main

import "fmt"

type Person struct {
	Name string
	Age  int
}

func main() {
	people := map[string]Person{"ada": {"Ada", 36}}
	p := people["ada"]
	p.Age++
	fmt.Println("map[string]Person, p := m[k]; p.Age++:", people["ada"].Age)
	for _, v := range people {
		v.Age = 99
	}
	fmt.Println("map[string]Person, range v.Age = 99: ", people["ada"].Age)

	ptrs := map[string]*Person{"ada": {"Ada", 36}}
	ptrs["ada"].Age++
	fmt.Println("map[string]*Person, m[k].Age++:      ", ptrs["ada"].Age)
}
