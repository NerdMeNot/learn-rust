package main

import (
	"cmp"
	"fmt"
	"slices"
	"strings"
)

// A struct of comparable fields gets == and works as a map key with nothing declared.
type Version struct {
	Major, Minor uint32
}

// Stringer is Go's Display: fmt calls String() for %v and %s.
func (v Version) String() string {
	return fmt.Sprintf("v%d.%d", v.Major, v.Minor)
}

// Ordering is not automatic: write the comparison.
func compare(a, b Version) int {
	if c := cmp.Compare(a.Major, b.Major); c != 0 {
		return c
	}
	return cmp.Compare(a.Minor, b.Minor)
}

func main() {
	a := Version{1, 10}
	b := Version{1, 9}
	fmt.Println("display:", a)
	fmt.Println("a == b:", a == b)
	fmt.Println("a > b:", compare(a, b) > 0)

	all := []Version{a, b, {}}
	slices.SortFunc(all, compare)
	names := []string{}
	for _, v := range all {
		names = append(names, v.String())
	}
	fmt.Println("sorted:", strings.Join(names, " "))

	seen := map[Version]bool{}
	for _, v := range []Version{a, b, {}, a} {
		seen[v] = true
	}
	fmt.Println("distinct:", len(seen))
}
