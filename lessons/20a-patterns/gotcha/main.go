package main

import (
	"fmt"
	"strings"
)

func main() {
	line := "mode"
	key, value, _ := strings.Cut(line, "=")
	fmt.Printf("%s -> %s\n", key, value)
}
