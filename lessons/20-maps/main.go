package main

import (
	"fmt"
	"strings"
)

func main() {
	ages := map[string]int{}
	ages["ana"] = 30
	ages["bo"] = 25

	if age, ok := ages["ana"]; ok {
		fmt.Println("ana is", age)
	} else {
		fmt.Println("no ana")
	}

	zed := ages["zed"] // 0 when missing
	fmt.Println("zed is", zed)

	counts := map[string]int{}
	for _, word := range strings.Split("a b a c a", " ") {
		counts[word]++
	}
	fmt.Println("a appears", counts["a"], "times")
}
