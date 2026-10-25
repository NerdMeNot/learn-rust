package main

import "fmt"

type Money struct {
	cents int64
}

// String() is Go's Display
func (m Money) String() string {
	return fmt.Sprintf("$%d.%02d", m.cents/100, m.cents%100)
}

type Item struct {
	Name string
	Qty  int
}

func main() {
	n := 42
	fmt.Printf("[%6d] [%-6d] [%06d]\n", n, n, n) // no centering verb in Go
	fmt.Printf("%.2f %e %#x %#b\n", 3.14159, 1234.5, n, n)

	price := Money{1999}
	fmt.Println("total:", price)

	label := fmt.Sprintf("%v each", price)
	fmt.Printf("%s (%d chars)\n", label, len(label))

	item := Item{"pen", 3}
	fmt.Printf("%d x %s\n", item.Qty, item.Name)
	fmt.Printf("%+v\n", item)
	fmt.Printf("%#v\n", item)
}
