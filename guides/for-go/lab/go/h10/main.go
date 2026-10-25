package main

import "fmt"

func main() {
	s := "héllo"
	fmt.Println(len(s), len([]rune(s)), s[1])
	fmt.Printf("%q\n", s[:2])
}
