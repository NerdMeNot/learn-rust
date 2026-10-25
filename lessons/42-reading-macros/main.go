package main

import (
	"fmt"
	"sort"
)

func main() {
	// A map literal is part of the language: no macro needed
	ages := map[string]int{"Ada": 36, "Linus": 28, "Grace": 45}

	names := make([]string, 0, len(ages))
	for name := range ages {
		names = append(names, name)
	}
	sort.Strings(names)
	for _, name := range names {
		fmt.Printf("%s: %d\n", name, ages[name])
	}
	fmt.Println("entries:", len(ages))
}
