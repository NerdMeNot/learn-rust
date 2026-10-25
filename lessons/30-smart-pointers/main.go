package main

import (
	"fmt"
	"strings"
)

type Log struct{ lines []string }

func main() {
	// a pointer is already shared: copying it copies the address
	log := &Log{}
	other := log

	log.lines = append(log.lines, "from log")
	other.lines = append(other.lines, "from other")

	fmt.Println("log:", strings.Join(log.lines, ", "))
	fmt.Println("same list:", log == other)
}
