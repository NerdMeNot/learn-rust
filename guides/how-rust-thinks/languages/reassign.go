package main

import "fmt"

func main() {
	count := 0
	count += 1 // fine: variables are mutable unless declared `const`
	const fixed = 0
	fixed += 1
	fmt.Println(count)
}
