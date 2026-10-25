package main

import (
	"fmt"
	"math"
)

func must(ok bool, what string) {
	if !ok {
		panic(what)
	}
}

func main() {
	sum := 0
	for i := range 4 { // Go 1.22 and later
		sum += i
	}
	must(sum == 6, "range over an int")

	n := 3
	for n > 0 {
		n--
	}
	must(n == 0, "for cond")

	count := 0
	for {
		count++
		if count == 3 {
			break
		}
	}
	must(count == 3, "for {}")

	hits := 0
outer:
	for a := 0; a < 3; a++ {
		for b := 0; b < 3; b++ {
			if b == 1 {
				continue outer
			}
			if a == 2 {
				break outer
			}
			hits++
		}
	}
	must(hits == 2, "labels")

	for i, v := range []string{"a", "b"} {
		must(v == []string{"a", "b"}[i], "range with index")
	}
	// range over an int: each pass has its own i, so changing it lasts one pass (like Rust)
	seen := []int{}
	for i := range 5 {
		if i == 1 {
			i += 2
		}
		seen = append(seen, i)
	}
	must(fmt.Sprint(seen) == "[0 3 2 3 4]", "range n copy")

	// three-clause for: the body's change carries into the next pass
	seen = nil
	for i := 0; i < 5; i++ {
		if i == 1 {
			i += 2
		}
		seen = append(seen, i)
	}
	must(fmt.Sprint(seen) == "[0 3 4]", "three-clause counter")

	// continue still runs the post statement
	steps := 0
	for i := 0; i < 3; i++ {
		if i == 1 {
			continue
		}
		steps++
	}
	must(steps == 2, "continue runs i++")

	// len is a signed int, so counting down with i >= 0 ends
	xs := []int{}
	down := 0
	for i := len(xs) - 1; i >= 0; i-- {
		down++
	}
	must(down == 0, "signed countdown on an empty slice")
	xs = []int{5, 6, 7}
	for i := len(xs) - 1; i >= 0; i-- {
		down++
	}
	must(down == 3, "signed countdown")

	// an unsigned counter wraps instead of going below zero
	var u uint = 0
	u--
	must(u == math.MaxUint, "uint wraps")
	fmt.Println("ok")
}
