package main

import "fmt"

func show(n int) string    { return fmt.Sprint("int ", n) }
func show(s string) string { return "text " + s }

func main() {
	fmt.Println(show(1), show("a"))
}
