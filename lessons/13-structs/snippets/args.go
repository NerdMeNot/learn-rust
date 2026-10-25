package main

import "fmt"

type User struct {
	Name string
	Age  int
}

func birthdayCopy(u User) { u.Age++ }

func birthdayPtr(u *User) { u.Age++ }

func main() {
	u := User{Name: "ana", Age: 30}
	birthdayCopy(u) // gets a copy; the caller keeps using u
	fmt.Println("after birthdayCopy:", u.Age)
	birthdayPtr(&u) // a pointer: may change u
	fmt.Println("after birthdayPtr:", u.Age)
}
