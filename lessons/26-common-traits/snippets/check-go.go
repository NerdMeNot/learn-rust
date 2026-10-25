package main

import (
	"cmp"
	"fmt"
	"math"
	"slices"
	"strconv"
)

type Version struct{ Major, Minor int }

func main() {
	// strconv.Atoi reports a bad number as a second return value
	if _, err := strconv.Atoi("4x"); err == nil {
		panic("Atoi accepted 4x")
	}
	n, err := strconv.Atoi("42")
	if err != nil || n != 42 {
		panic("Atoi")
	}
	// cmp.Compare gives -1, 0 or 1; slices.SortFunc takes such a function
	if cmp.Compare(1, 2) != -1 || cmp.Compare("b", "a") != 1 || cmp.Compare(3, 3) != 0 {
		panic("Compare")
	}
	vs := []Version{{1, 10}, {1, 9}, {0, 0}}
	slices.SortFunc(vs, func(a, b Version) int {
		return cmp.Or(cmp.Compare(a.Major, b.Major), cmp.Compare(a.Minor, b.Minor))
	})
	if vs[0] != (Version{0, 0}) || vs[2] != (Version{1, 10}) {
		panic("SortFunc")
	}
	// assignment copies a struct
	a := Version{1, 2}
	b := a
	b.Major = 9
	if a.Major != 1 {
		panic("copy")
	}
	// cmp.Compare accepts floats and sorts NaN before every number; NaN equals itself there
	nan := math.NaN()
	if cmp.Compare(nan, 1.0) != -1 || cmp.Compare(nan, nan) != 0 || cmp.Compare(1.0, nan) != 1 {
		panic("Compare NaN")
	}
	// float64 is allowed as a map key, but a NaN key is never found again
	m := map[float64]int{1.5: 1}
	m[nan] = 1
	m[nan] = 2
	if m[1.5] != 1 || len(m) != 3 || m[nan] != 0 {
		panic("float64 key")
	}
	fmt.Println("all checks passed")
}
