package main

import (
	"errors"
	"fmt"
	"strconv"
)

func must(ok bool, what string) {
	if !ok {
		panic(what)
	}
}

// A package-level variable that any function may change.
var visits int

func visit() { visits++ }

func parse(s string) (int, error) {
	n, err := strconv.Atoi(s)
	if err != nil {
		return 0, err
	}
	return n + 1, nil
}

type Speaker interface{ Speak() string }

type Dog struct{}

func (Dog) Speak() string { return "woof" }

type Animal struct{ Name string }

type Pet struct {
	Animal
	Tricks int
}

type User struct {
	Name string
	Age  int
}

func main() {
	// i++ and i-- are statements
	i := 5
	i++
	i--
	must(i == 5, "inc")

	// the three-part for, range over an integer (Go 1.22), a condition, and no condition
	a, b := 0, 0
	for k := 0; k < 3; k++ {
		a += k
	}
	for k := range 3 {
		b += k
	}
	must(a == 3 && b == 3, "for")
	c := 3
	for c > 0 {
		c--
	}
	loops := 0
	for {
		loops++
		if loops == 2 {
			break
		}
	}
	must(c == 0 && loops == 2, "for forms")

	// range over a slice with an index and a value
	total := 0
	for idx, v := range []int{5, 6} {
		total += idx*100 + v
	}
	must(total == 5+106, "range")

	// nil, and the comma-ok form
	var p *int
	var m map[string]int
	must(p == nil && m == nil, "nil")
	ages := map[string]int{"ada": 36}
	v, ok := ages["ada"]
	_, missing := ages["bob"]
	must(v == 36 && ok && !missing, "comma ok")

	// switch: several values in one case, no fallthrough, and no subject
	out := ""
	switch 2 {
	case 1, 2:
		out += "small"
	case 3:
		out += "three"
	}
	must(out == "small", "switch")
	x := 9
	label := ""
	switch {
	case x > 5:
		label = "big"
	default:
		label = "small"
	}
	must(label == "big", "tagless switch")

	// fallthrough has to be written out
	out = ""
	switch 1 {
	case 1:
		out += "a"
		fallthrough
	case 2:
		out += "b"
	}
	must(out == "ab", "fallthrough")

	// goto
	n := 0
again:
	n++
	if n < 3 {
		goto again
	}
	must(n == 3, "goto")

	// defer runs when the function returns
	trace := ""
	func() {
		defer func() { trace += "deferred" }()
		trace += "body,"
	}()
	must(trace == "body,deferred", "defer")

	// swapping with a comma assignment
	s, t := 1, 2
	s, t = t, s
	must(s == 2 && t == 1, "swap")

	// the rest of a struct literal is zero
	u := User{Name: "a"}
	must(u.Age == 0, "zero value")

	// embedding promotes the fields, and an interface is satisfied without a declaration
	pet := Pet{Animal{"Rex"}, 3}
	must(pet.Name == "Rex", "embedding")
	var sp Speaker = Dog{}
	must(sp.Speak() == "woof", "interface")

	// errors are values
	_, err := parse("x")
	var numErr *strconv.NumError
	must(err != nil && errors.As(err, &numErr), "err")
	pv, err := parse("41")
	must(err == nil && pv == 42, "ok")

	// a mutable package-level variable
	visit()
	visit()
	must(visits == 2, "global")

	// typed numbers convert explicitly, and a constant adopts the type
	var small int32 = 5
	var big int64 = 7
	must(int64(small)+big == 12, "convert")
	d := 1.5
	must(d*2 == 3.0, "untyped constant")

	fmt.Println("go: all checks passed")
}
