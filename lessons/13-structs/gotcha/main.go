package main

import "fmt"

type User struct {
	Name   string
	Age    int
	Active bool
}

func main() {
	u := User{Name: "ana"}
	fmt.Println(u.Name, u.Age, u.Active)
}
