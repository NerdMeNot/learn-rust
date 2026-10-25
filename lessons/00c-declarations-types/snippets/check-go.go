package main

import "strconv"

type P struct{ X, Y int }

type Stack[T any] struct{ items []T }

func add(a, b int) int { return a + b }

func ident[T any](x T) T { return x }

func must(ok bool, what string) {
	if !ok {
		panic(what)
	}
}

const Max = 100

func main() {
	x := 5
	var y int = 5
	var z int
	must(x == y && z == 0, "zero value")

	var f float64 = Max // an untyped constant fits a float: Rust's `const` has no such thing
	must(f == 100, "untyped const")

	a, b := 1, "two"
	must(a == 1 && b == "two", "multi")

	arr := [3]int{1, 2, 3}
	sl := []int{1, 2, 3}
	must(len(arr) == 3 && len(sl) == 3, "arrays")

	p := P{1, 2}
	must(p.X+p.Y == 3, "struct")

	inc := func(x int) int { return x + 1 }
	must(inc(1) == 2 && add(1, 2) == 3, "func")

	n := 7
	price := 2.9
	must(float64(n) == 7.0 && int(price) == 2 && int(-price) == -2, "conversion")

	r := 'R'
	must(r == 82, "rune")
	raw := `C:\temp`
	must(len(raw) == 7, "raw string")
	must(0xff == 255 && 1_000 == 1000 && 0b101 == 5 && 017 == 15, "literals")

	s := Stack[int]{}
	must(len(s.items) == 0 && ident[int](5) == 5, "generics")
	m := map[string]int{}
	must(len(m) == 0, "map")
	big := 1 << 40 // Go's int is 64 bits here: this runs
	must(big == 1099511627776 && strconv.IntSize == 64, "int is 64 bits")
	must('R'+1 == 83, "a rune is a number")
	wide := 300
	must(uint8(wide) == 44 && 0o17 == 15, "narrowing wraps, 0o17")
	var name string = "Ada"
	must(len(name) == 3 && p.X == 1, "string, struct")
	v, err := strconv.Atoi("42")
	must(v == 42 && err == nil, "Atoi")
}
