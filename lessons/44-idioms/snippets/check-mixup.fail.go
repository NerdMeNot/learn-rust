package main

import "fmt"

type UserID int
type OrderID int

func loadUser(id UserID) string {
	return fmt.Sprint("user #", int(id))
}

func main() {
	order := OrderID(7)
	fmt.Println(loadUser(order))
}
