package main

import (
	"fmt"
	"os"
)

func main() {
	const n = len(os.Args)
	fmt.Println(n)
}
