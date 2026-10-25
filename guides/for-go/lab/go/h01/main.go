package main

import "fmt"

type Person struct {
	Name string
	Age  int
}

func main() {
	people := []Person{{"ana", 30}, {"bo", 41}}
	for _, p := range people {
		p.Age++ // p is a copy of the element
	}
	fmt.Println(people)
	for i := range people {
		people[i].Age++
	}
	fmt.Println(people)
}
