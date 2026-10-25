package main

import "fmt"

type Meters float64
type Feet float64

func describe(m Meters) string { return fmt.Sprintf("%v meters", float64(m)) }

func main() {
	altitude := Feet(1200)
	fmt.Println(describe(altitude))
}
