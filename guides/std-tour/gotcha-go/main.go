package main

import "fmt"

func main() {
	names := []string{"ana", "bo"} // slices: built in
	ages := map[string]int{}       // maps: built in too
	ages["ana"] = 30
	fmt.Println(len(names), len(ages))
}
