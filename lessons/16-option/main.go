package main

import "fmt"

type User struct {
	Name string
}

func findUser(id int) *User {
	if id == 1 {
		return &User{Name: "ana"}
	}
	return nil
}

func main() {
	if u := findUser(1); u != nil {
		fmt.Println("found", u.Name)
	} else {
		fmt.Println("no user")
	}

	if u := findUser(2); u != nil {
		fmt.Println("found", u.Name)
	} else {
		fmt.Println("no user")
	}
}
