package main

import (
	"fmt"
	"strings"
)

func main() {
	line := "ana,30,lisbon"
	parts := strings.Split(line, ",")
	name := parts[0]
	fmt.Println(name)
}
