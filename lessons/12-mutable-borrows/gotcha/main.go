package main

import "fmt"

func main() {
	scores := []int{10, 20, 30}
	first := &scores[0]
	last := &scores[2]
	*first += 1
	*last += 1
	fmt.Println(scores)
}
