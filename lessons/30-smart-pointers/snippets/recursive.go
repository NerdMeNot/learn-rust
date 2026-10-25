package main

import "fmt"

// the field is a pointer you write yourself; nil ends the tree
type Expr struct {
	Value       int
	Left, Right *Expr
}

func main() {
	tree := &Expr{1, &Expr{2, nil, nil}, &Expr{3, nil, nil}}
	fmt.Println("left child:", tree.Left.Value)
}
