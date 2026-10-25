package main

import (
	"fmt"
	"strconv"
)

func must(ok bool, what string) {
	if !ok {
		panic(what)
	}
}

func add(a, b int) int { return a + b }

func double(s string) (int, error) {
	n, err := strconv.Atoi(s)
	if err != nil {
		return 0, err
	}
	return n * 2, nil
}

type Point struct{ X int }

func main() {
	x := 5
	x = 6
	const y = 5
	must(x == 6 && y == 5, "variables")
	must(add(2, 3) == 5, "func")

	count := 0
	for i := 0; i < 3; i++ {
		count += i
	}
	must(count == 3, "counted loop")

	words := []string{"a", "b"}
	joined := ""
	for _, w := range words {
		joined += w
	}
	must(joined == "ab", "range")

	n := 1
	var s string
	switch n {
	case 1:
		s = "one"
	default:
		s = "many"
	}
	must(s == "one", "switch")

	inc := func(x int) int { return x + 1 }
	must(inc(1) == 2, "func literal")

	name, age := "Ana", 30
	fmt.Printf("%s is %d\n", name, age)

	v, err := double("21")
	must(v == 42 && err == nil, "double")
	_, err = double("x")
	must(err != nil, "double error")
	fmt.Println("all go cells compile and hold")
}
