package main

import (
	"fmt"
	"slices"
	"sort"
)

func must(ok bool, what string) {
	if !ok {
		panic(what)
	}
}

func main() {
	v := []int{3, 1, 2}
	v = append(v, 4)
	w := []int{7, 8}
	v = append(v, w...)
	must(slices.Equal(v, []int{3, 1, 2, 4, 7, 8}), "append")
	must(len(v) == 6 && cap(v) >= 6, "len cap")

	z := make([]int, 3)
	must(slices.Equal(z, []int{0, 0, 0}), "make n")
	must(len(make([]int, 0, 10)) == 0 && cap(make([]int, 0, 10)) == 10, "make cap")

	must(v[len(v)-1] == 8, "last")
	v = v[:len(v)-1]
	must(len(v) == 5, "pop")

	v = append(v[:1], v[2:]...)
	must(slices.Equal(v, []int{3, 2, 4, 7}), "remove")
	v = slices.Insert(v, 1, 5)
	must(slices.Equal(v, []int{3, 5, 2, 4, 7}), "insert")

	s := slices.Clone(v)
	slices.Sort(s)
	must(slices.Equal(s, []int{2, 3, 4, 5, 7}), "sort")
	pos, found := slices.BinarySearch(s, 4)
	must(pos == 2 && found, "binary search")
	pos, found = slices.BinarySearch(s, 6)
	must(pos == 4 && !found, "binary search miss")
	must(slices.Contains(s, 5) && slices.Index(s, 5) == 3 && slices.Index(s, 99) == -1, "contains index")

	names := []string{"kiwi", "fig", "banana"}
	sort.Slice(names, func(i, j int) bool { return len(names[i]) < len(names[j]) })
	must(slices.Equal(names, []string{"fig", "kiwi", "banana"}), "sort.Slice")

	slices.Reverse(s)
	must(slices.Equal(s, []int{7, 5, 4, 3, 2}), "reverse")
	runs := slices.Compact([]int{1, 1, 2, 2, 1})
	must(slices.Equal(runs, []int{1, 2, 1}), "compact")

	window := s[1:3]
	window[0] = 99
	must(s[1] == 99, "a subslice shares the array")
	dst := make([]int, 2)
	must(copy(dst, []int{1, 2, 3}) == 2 && slices.Equal(dst, []int{1, 2}), "copy copies the shorter length")
	cl := slices.Clone(dst)
	cl[0] = 9
	must(dst[0] == 1, "Clone is independent")
	dst = dst[:0]
	must(len(dst) == 0, "s[:0] empties")
	arr := [3]int{1, 2, 3}
	arr2 := arr
	arr2[0] = 9
	must(arr[0] == 1, "arrays copy on assignment")
	fmt.Println("all checks passed")
}
