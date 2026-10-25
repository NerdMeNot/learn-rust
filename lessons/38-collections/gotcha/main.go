package main

import (
	"fmt"
	"strings"
)

func main() {
	counts := map[string]int{}
	counts["cat"] = 0
	for _, w := range strings.Split("cat cat", " ") {
		counts[w] += 1
	}
	fmt.Println(counts)
}
