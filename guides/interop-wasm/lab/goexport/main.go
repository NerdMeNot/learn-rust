package main

// The Go counterpart of adder's add: a function the host can call (go:wasmexport, Go 1.24+).

//go:wasmexport add
func add(a, b int32) int32 { return a + b }

// main is required but is not run in this mode.
func main() {}
