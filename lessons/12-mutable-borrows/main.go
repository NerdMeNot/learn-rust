package main

import "fmt"

func addBonus(scores *[]int, bonus int) {
	for i := range *scores {
		(*scores)[i] += bonus
	}
	*scores = append(*scores, 0)
}

func main() {
	scores := []int{10, 20}
	addBonus(&scores, 5)
	fmt.Println(scores)
}
