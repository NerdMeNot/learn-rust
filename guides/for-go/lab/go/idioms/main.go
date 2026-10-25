package main

import (
	"errors"
	"fmt"
	"sort"
	"strconv"
	"strings"
)

type Celsius float64

type Counter struct{ n int }

func (c *Counter) Inc() { c.n++ }

func main() {
	x := 5
	x = x + 1
	fmt.Println("reassign:", x)

	var zero int
	fmt.Println("zero:", zero)

	sum := 0
	for i := 0; i < 3; i++ {
		sum += i
	}
	fmt.Println("range:", sum)

	words := []string{"ana", "bo", "cy"}
	total := 0
	for i, w := range words {
		total += i * len(w)
	}
	fmt.Println("weighted:", total)

	day := "sat"
	kind := "weekday"
	switch day {
	case "sat", "sun":
		kind = "weekend"
	}
	fmt.Println("kind:", kind)

	ages := map[string]int{"ana": 30}
	if v, ok := ages["ana"]; ok {
		fmt.Println("lookup: found", v)
	}

	counts := map[string]int{}
	counts["the"]++
	counts["the"]++
	fmt.Println("count:", counts["the"])

	v := []int{3, 1}
	v = append(v, 2)
	fmt.Println("len:", len(v))

	s := "héllo"
	fmt.Println("bytes:", len(s))
	fmt.Println("chars:", len([]rune(s)))
	fmt.Println("fields:", len(strings.Fields("a b")))

	if n, err := strconv.Atoi("42"); err == nil {
		fmt.Println("parsed: ok", n)
	}

	fmt.Println("formatted:", fmt.Sprintf("%s-%d", "ana", 7))

	sort.Ints(v)
	fmt.Println("smallest:", v[0])

	fmt.Println("upper:", strings.ToUpper("ana"))

	a, b := 1, 2
	a, b = b, a
	fmt.Println("swapped:", a, b)

	p, q := 3, 5
	fmt.Println("min:", min(p, q))

	c := Celsius(21.5)
	fmt.Println("newtype:", float64(c))

	e := errors.New("bad input")
	fmt.Println("error:", e)

	var ctr Counter
	ctr.Inc()
	fmt.Println("method:", ctr.n)
}
