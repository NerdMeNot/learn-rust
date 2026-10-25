package main

import "fmt"

type Version struct {
	Major, Minor uint32
}

func main() {
	notes := map[Version]string{}
	notes[Version{1, 2}] = "stable"
	found, ok := notes[Version{1, 2}]
	fmt.Println("found:", found, ok)
}
