package main

import "fmt"

type User struct{ Name string }

func main() {
	ages := map[string]int{"ana": 31, "zed": 0}

	// The (value, ok) idiom: ok says whether the key was there.
	v, ok := ages["zed"]
	fmt.Println("zed:", v, ok)
	v, ok = ages["bo"]
	fmt.Println("bo:", v, ok)

	// Drop the ok and a missing key looks like a stored 0.
	fmt.Println("zed without ok:", ages["zed"])
	fmt.Println("bo without ok:", ages["bo"])

	// A pointer's zero value is nil, and the compiler lets you use it.
	var u *User
	fmt.Println("u == nil:", u == nil)
}
