package main

import (
	"fmt"
	"runtime"
)

// entry returns the address where the running copy of Id starts.
//
//go:noinline
func Id[T any](x T) (T, uintptr) {
	pc, _, _, _ := runtime.Caller(0)
	return x, runtime.FuncForPC(pc).Entry()
}

func main() {
	i, s := 1, "x"
	_, forIntPtr := Id(&i)
	_, forStrPtr := Id(&s)
	_, forInt := Id(1)
	_, forInt64 := Id(int64(1))
	fmt.Println("Id[*int] and Id[*string] run the same code:", forIntPtr == forStrPtr)
	fmt.Println("Id[int] and Id[int64] run the same code:", forInt == forInt64)
}
