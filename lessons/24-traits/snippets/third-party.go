package main

import (
	"fmt"
	"strings"
)

func main() {
	var b strings.Builder
	b.WriteString("hello")

	// strings.Builder and fmt.Stringer come from the standard library, and
	// neither mentions the other. The Builder has String(), so it fits.
	var s fmt.Stringer = &b
	fmt.Println(s.String())
}
