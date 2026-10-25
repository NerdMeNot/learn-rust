package main

import "fmt"

// The zero value of a T carries its type, which %T can print.
func Name[T any]() string {
	var zero T
	return fmt.Sprintf("%T", zero)
}

type MyInt int

func main() {
	fmt.Println("Name[int]():", Name[int]())
	fmt.Println("Name[[]string]():", Name[[]string]())
	fmt.Println("Name[MyInt]():", Name[MyInt]())
	// The zero value of an interface type is nil, which has no type to print.
	fmt.Println("Name[error]():", Name[error]())
}
