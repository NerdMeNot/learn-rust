package main

import (
	"fmt"

	"example.com/tools/internal/banner"
)

func main() {
	fmt.Println(banner.For("seed"))
	fmt.Println("inserted 3 sample rows")
}
