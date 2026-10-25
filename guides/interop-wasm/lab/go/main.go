package main

import (
	"fmt"
	"os"
)

func main() {
	fmt.Println("hello from Go, args:", os.Args[1:])
}
