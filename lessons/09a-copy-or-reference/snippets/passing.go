package main

import "fmt"

type Team struct {
	Name    string
	Members []string
}

type Counter struct{ n int }

func (c Counter) String() string { return fmt.Sprint("n=", c.n) }

func setFirst(s []int) { s[0] = 42 }

func grow(s []int) { s = append(s, 99) }

func fill(m map[string]int) { m["a"] = 1 }

func zeroFirst(a [3]int) { a[0] = 0 }

func rename(t Team) {
	t.Name = "renamed"
	t.Members[0] = "renamed"
}

func main() {
	nums := []int{1, 2, 3}
	setFirst(nums)
	grow(nums)
	fmt.Println("slice after setFirst and grow:", nums, "len", len(nums))

	m := map[string]int{}
	fill(m)
	fmt.Println("map after fill:               ", m)

	arr := [3]int{1, 2, 3}
	zeroFirst(arr)
	fmt.Println("array after zeroFirst:        ", arr)

	team := Team{"red", []string{"ana"}}
	rename(team)
	fmt.Println("struct after rename:          ", team)

	// an interface value holds its own copy of a struct
	c := Counter{1}
	var s fmt.Stringer = c
	c.n = 99
	fmt.Println("interface:", s, " variable:", c)
}
