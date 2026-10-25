package main

import "fmt"

type User struct{ Name string }

func findUser(id int) *User {
	if id == 1 {
		return &User{Name: "ana"}
	}
	return nil
}

func main() {
	u := findUser(2)
	fmt.Println(u.Name)
}
