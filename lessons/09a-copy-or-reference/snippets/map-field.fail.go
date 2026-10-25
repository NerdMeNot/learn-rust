package main

import "fmt"

type Person struct {
	Name string
	Age  int
}

func main() {
	people := map[string]Person{"ada": {"Ada", 36}}
	people["ada"].Age++
	fmt.Println(people)
}
