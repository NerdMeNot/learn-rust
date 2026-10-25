package main

import "fmt"

type User struct {
	Name   string
	Age    int
	Active bool
}

func main() {
	u := User{Name: "ana", Age: 30, Active: true}
	fmt.Printf("%s is %d, active=%t\n", u.Name, u.Age, u.Active)
	fmt.Printf("%+v\n", u)
}
