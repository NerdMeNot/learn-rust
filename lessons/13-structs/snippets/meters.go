package main

import "fmt"

type Meters float64

func describe(m Meters) string { return fmt.Sprintf("%v meters", float64(m)) }

func main() {
	// An untyped constant is accepted as Meters.
	fmt.Println(describe(1200.0))
}
