package main

import "fmt"

type User struct{ Name string }

func find(id int) *User {
	if id == 1 {
		return &User{"ana"}
	}
	return nil
}

func main() {
	u := find(2)
	fmt.Println(u.Name)
}
