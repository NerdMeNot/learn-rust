package main

import (
	"fmt"
	"strings"
	"testing"
)

var sink []string
var ints []int

func main() {
	line := "alpha beta gamma delta epsilon zeta eta theta"
	fmt.Println("strings.Split:           ", testing.AllocsPerRun(100, func() { sink = strings.Split(line, " ") }))
	fmt.Println("1000 appends, no hint:   ", testing.AllocsPerRun(100, func() {
		var v []int
		for i := 0; i < 1000; i++ {
			v = append(v, i)
		}
		ints = v
	}))
	fmt.Println("1000 appends, capacity:  ", testing.AllocsPerRun(100, func() {
		v := make([]int, 0, 1000)
		for i := 0; i < 1000; i++ {
			v = append(v, i)
		}
		ints = v
	}))
}
