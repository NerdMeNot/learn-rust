package main

import (
	"fmt"
	"unsafe"
)

func main() {
	a := []int{1, 2, 3}
	b := a // copies the slice header, not the elements
	b[0] = 99

	fmt.Println("a[0] after b[0] = 99:", a[0])
	fmt.Println("len and cap of a:", len(a), cap(a))
	fmt.Println("size of a slice header:", unsafe.Sizeof(a), "bytes")
	fmt.Println("a and b start at the same element:", &a[0] == &b[0])
	fmt.Printf("addresses: %p %p\n", &a[0], &b[0])

	b = append(b, 4) // no room left: append moves b to a new array
	b[0] = -1
	fmt.Println("after append: a[0] =", a[0], ", start at the same element:", &a[0] == &b[0])
}
