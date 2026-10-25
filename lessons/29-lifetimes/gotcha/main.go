package main

import (
	"fmt"
	"strings"
)

func longest(a, b string) string {
	if len(a) >= len(b) {
		return a
	}
	return b
}

// join builds a new string at run time: a heap object, not a literal.
func join(parts ...string) string {
	return strings.Join(parts, " ")
}

func main() {
	s1 := join("a", "long", "first", "string")
	var result, other string
	{
		s2 := join("short", "string")
		result = longest(s1, s2)
		s3 := join("an", "even", "longer", "second", "string")
		other = longest(s1, s3)
	}
	fmt.Println("longest:", result)
	fmt.Println("with a longer s3:", other)
}
