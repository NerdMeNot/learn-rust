package main

import "fmt"

// What other languages write with ?:, i++, null and switch, written in Go.
func size(n int) string {
	switch n {
	case 1, 2:
		return "small"
	case 3:
		return "medium"
	default:
		return "large"
	}
}

func firstEven(items []int) (int, bool) {
	for _, x := range items {
		if x%2 == 0 {
			return x, true
		}
	}
	return 0, false
}

func main() {
	total := 0
	for i := 0; i < 4; i++ {
		total += i
	}
	label := "small"
	if total > 5 {
		label = "big"
	}
	fmt.Printf("total: %d (%s)\n", total, label)
	fmt.Printf("size: %s %s\n", size(2), size(9))
	if n, ok := firstEven([]int{3, 5, 8}); ok {
		fmt.Printf("first even: %d\n", n)
	} else {
		fmt.Println("no even number")
	}
	n, ok := firstEven([]int{1, 3})
	if !ok {
		n = -1
	}
	fmt.Printf("none: %d\n", n)
	wide := int64(total) * 1_000_000_000
	fmt.Printf("wide: %d\n", wide)
}
