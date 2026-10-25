package main

import "fmt"

func must(ok bool, what string) {
	if !ok {
		panic(what)
	}
}

func main() {
	// Package-level functions can be used before their definition.
	must(later(2) == 4, "order")

	// Parameters of one type can share it.
	must(add(2, 3) == 5, "grouped parameters")

	// Multiple results.
	q, r := divmod(7, 2)
	must(q == 3 && r == 1, "divmod")

	// Variadic.
	must(sum(1, 2, 3) == 6 && sum() == 0, "variadic")
	xs := []int{4, 5}
	must(sum(xs...) == 9, "spread")

	// Function types and literals.
	var f func(int) int = later
	must(f(5) == 10, "func value")
	g := func(n int) int { return n + 1 }
	must(g(1) == 2, "func literal")

	// Named results with a bare return.
	must(named() == 3, "named result")

	// A func literal captures freely.
	offset := 5
	h := func(n int) int { return n + offset }
	must(h(1) == 6, "closure captures")

	// Discarding a result and an unused parameter.
	q2, _ := divmod(7, 2)
	must(q2 == 3, "blank identifier")
	ignore(1)

	// A string or a struct passed by value is copied; the caller keeps its own.
	name := "go"
	must(length(name) == 2 && name == "go", "string copy")
	box := Box{n: 1}
	grow(box)
	must(box.n == 1, "struct copy")

	// A pointer parameter writes through.
	x := 1
	inc(&x)
	must(x == 2, "pointer parameter")

	// A method.
	c := Counter{}
	c.Inc()
	must(c.n == 1, "method")

	fmt.Println("all checks passed")
}

func ignore(_ int) {}

func length(s string) int { return len(s) }

type Box struct{ n int }

func grow(b Box) { b.n++ }

func inc(p *int) { *p++ }

type Counter struct{ n int }

func (c *Counter) Inc() { c.n++ }

func later(n int) int { return n * 2 }

func add(a, b int) int { return a + b }

func divmod(a, b int) (int, int) { return a / b, a % b }

func sum(nums ...int) int {
	total := 0
	for _, n := range nums {
		total += n
	}
	return total
}

func named() (n int) {
	n = 3
	return
}
