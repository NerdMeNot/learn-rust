package main

import "fmt"

type Version struct {
	Major, Minor uint32
}

func main() {
	a := Version{1, 2}
	b := Version{1, 2}
	fmt.Println("a == b:", a == b)

	seen := map[Version]bool{a: true}
	fmt.Println("key found:", seen[b])

	// Any struct prints, with no setup (Rust needs #[derive(Debug)]).
	fmt.Printf("%v and %+v\n", a, a)
}
