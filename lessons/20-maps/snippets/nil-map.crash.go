package main

import "fmt"

func main() {
	var m map[string]int // nil: declared, never made
	fmt.Println("read:", m["a"], "len:", len(m))
	m["a"] = 1
	fmt.Println("not reached")
}
