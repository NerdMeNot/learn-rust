package main

import (
	"fmt"
	"strings"
)

type Item struct {
	name  string
	count int
}

type Inventory struct {
	items []Item
	log   []string
}

func (inv *Inventory) check() {
	for _, item := range inv.items {
		if item.count == 0 {
			inv.log = append(inv.log, "out of stock: "+item.name)
		}
	}
}

func show(xs []string) string {
	quoted := make([]string, len(xs))
	for i, x := range xs {
		quoted[i] = fmt.Sprintf("%q", x)
	}
	return "[" + strings.Join(quoted, ", ") + "]"
}

func main() {
	inv := &Inventory{items: []Item{{"bolt", 0}, {"nut", 12}, {"washer", 0}}}
	inv.check()
	fmt.Println("log:", show(inv.log))
}
