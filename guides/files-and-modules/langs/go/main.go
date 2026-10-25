package main

import (
	"fmt"

	"example.com/app/db"
)

func main() {
	fmt.Println(helper())
	fmt.Println(db.Name())
}
