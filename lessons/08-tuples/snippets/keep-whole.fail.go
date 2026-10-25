package main

import "fmt"

func pair() (int, int) { return 3, 4 }

func main() {
	r := pair()
	fmt.Println(r)
}
