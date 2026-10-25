// Backs the Go cells of the tables in lesson.mdx. Never shown.
package main

import (
	"fmt"
	"slices"
)

func main() {
	v := []string{"ana", "bo"}
	var got []string
	for i, n := range v {
		got = append(got, fmt.Sprint(i, n))
	}
	if fmt.Sprint(got) != "[0ana 1bo]" {
		panic(got)
	}
	m := map[string]int{"a": 1}
	for k, c := range m {
		if k != "a" || c != 1 {
			panic("map")
		}
	}
	var rev []string
	for i := len(v) - 1; i >= 0; i-- {
		rev = append(rev, v[i])
	}
	if fmt.Sprint(rev) != "[bo ana]" {
		panic(rev)
	}
	// filter in place: slices.DeleteFunc, or the out := v[:0] idiom
	w := []int{1, 2, 3, 4, 5, 6}
	w = slices.DeleteFunc(w, func(x int) bool { return x%2 == 0 })
	if fmt.Sprint(w) != "[1 3 5]" {
		panic(w)
	}
	u := []int{1, 2, 3, 4, 5, 6}
	out := u[:0]
	for _, x := range u {
		if x%2 != 0 {
			out = append(out, x)
		}
	}
	if fmt.Sprint(out) != "[1 3 5]" {
		panic(out)
	}
	// delete during range over a map is allowed
	d := map[string]int{"a": 1, "b": 2, "c": 3}
	for k := range d {
		delete(d, k)
	}
	if len(d) != 0 {
		panic(d)
	}
	fmt.Println("ok")
}
