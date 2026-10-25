package main

import (
	"fmt"
	"math"
)

func main() {
	m := map[float64]string{}
	m[math.NaN()] = "first"
	m[math.NaN()] = "second"
	_, found := m[math.NaN()]
	fmt.Println("len:", len(m), "found:", found)
}
