package main

import (
	"fmt"
	"os"
)

func main() {
	defer fmt.Println("deferred call ran")
	fmt.Println("calling os.Exit")
	os.Exit(0)
}
