package main

import (
	"fmt"

	"example.com/tools/internal/banner"
)

func main() {
	fmt.Println(banner.For("server"))
	fmt.Println("serving requests...")
}
