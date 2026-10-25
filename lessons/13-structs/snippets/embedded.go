package main

import "fmt"

type User struct {
	Name string
	Tags []string
}

type Admin struct {
	User
	Level int
}

func main() {
	a := Admin{User: User{Name: "ana", Tags: []string{"staff"}}, Level: 2}
	fmt.Println("promoted field:", a.Name, "same as", a.User.Name)

	b := a // a copy of the struct, but not of what its slice points at
	b.Name = "bob"
	b.Tags[0] = "guest"
	fmt.Println("a:", a.Name, a.Tags)
	fmt.Println("b:", b.Name, b.Tags)
}
