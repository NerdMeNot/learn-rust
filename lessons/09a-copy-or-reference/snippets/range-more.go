package main

import "fmt"

type Counter struct{ n int }

func (c Counter) Value() int { return c.n }
func (c *Counter) Inc()      { c.n++ }

func main() {
	// range over an array copies the array first; range &arr and a slice do not
	arr := [3]int{1, 2, 3}
	for i, v := range arr {
		arr[2] = 100
		if i == 2 {
			fmt.Println("range arr saw: ", v)
		}
	}
	arr = [3]int{1, 2, 3}
	for i, v := range &arr {
		arr[2] = 100
		if i == 2 {
			fmt.Println("range &arr saw:", v)
		}
	}
	sl := []int{1, 2, 3}
	for i, v := range sl {
		sl[2] = 100
		if i == 2 {
			fmt.Println("range slice saw:", v)
		}
	}

	// A pointer method on the range variable compiles, and changes the copy
	counters := []Counter{{}, {}}
	for _, c := range counters {
		c.Inc()
	}
	fmt.Println("after c.Inc() on the range variable:", counters)
	for i := range counters {
		counters[i].Inc()
	}
	fmt.Println("after counters[i].Inc():             ", counters)
	for _, c := range counters {
		_ = c.Value() // a value receiver always gets a copy
	}
}
