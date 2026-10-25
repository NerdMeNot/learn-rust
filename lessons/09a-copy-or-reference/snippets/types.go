package main

import "fmt"

type Point struct{ X int }

func report(kind string, changedOriginal bool) {
	if changedOriginal {
		fmt.Printf("%-8s shares: the other name sees the change\n", kind)
	} else {
		fmt.Printf("%-8s copy:   the other name is untouched\n", kind)
	}
}

func main() {
	n := 1
	m := n
	m = 99
	report("int", n == 99)

	s := "go"
	t := s
	t = "changed"
	report("string", s == "changed")

	arr := [2]int{1, 2}
	arr2 := arr
	arr2[0] = 99
	report("array", arr[0] == 99)

	pt := Point{1}
	pt2 := pt
	pt2.X = 99
	report("struct", pt.X == 99)

	sl := []int{1, 2}
	sl2 := sl
	sl2[0] = 99
	report("slice", sl[0] == 99)

	mp := map[string]int{"a": 1}
	mp2 := mp
	mp2["a"] = 99
	report("map", mp["a"] == 99)

	p := &Point{1}
	p2 := p
	p2.X = 99
	report("pointer", p.X == 99)

	ch := make(chan int, 1)
	ch2 := ch
	ch2 <- 7
	report("chan", len(ch) == 1)

	count := 0
	f := func() { count++ }
	f2 := f
	f2()
	report("func", count == 1)

	_ = m
	_ = t
}
